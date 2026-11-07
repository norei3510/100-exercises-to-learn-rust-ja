<a id="scoped-threads"></a>

# スコープ付きスレッド

これまで議論してきたライフタイムに関する問題は、すべて共通の原因があります。
それは、生成されたスレッドが親スレッドよりも長く生存する可能性があることです。\
**スコープ付きスレッド**を使用することで、この問題を回避できます。

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

std::thread::scope(|scope| {
    scope.spawn(|| {
        let first = &v[..midpoint];
        println!("Here's the first half of v: {first:?}");
    });
    scope.spawn(|| {
        let second = &v[midpoint..];
        println!("Here's the second half of v: {second:?}");
    });
});

println!("Here's v: {v:?}");
```


ここで何が起きているのか詳しく見ていきましょう。

<a id="scope"></a>

## `scope`

`std::thread::scope` 関数は新しい**スコープ**を作成します。\
`std::thread::scope` はクロージャを引数として受け取ります。そのクロージャは `Scope` インスタンスを唯一の引数として受け取ります。

<a id="scoped-spawns"></a>

## スコープ付きスポーン

`Scope` は `spawn` メソッドを提供しています。\
`std::thread::spawn` とは異なり、`Scope` を使用して生成されたすべてのスレッドは、
スコープが終了したときに**自動的にジョイン**されます。

先ほどの例を `std::thread::spawn` に「書き換える」と、
次のようになります。

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

let handle1 = std::thread::spawn(|| {
    let first = &v[..midpoint];
    println!("Here's the first half of v: {first:?}");
});
let handle2 = std::thread::spawn(|| {
    let second = &v[midpoint..];
    println!("Here's the second half of v: {second:?}");
});

handle1.join().unwrap();
handle2.join().unwrap();

println!("Here's v: {v:?}");
```


<a id="borrowing-from-the-environment"></a>

## 環境からの借用

ただし、書き換えた例はコンパイルできません。コンパイラは、
`&v` のライフタイムが `'static` ではないため、生成されたスレッドから
使用できないと警告します。

これは `std::thread::scope` の問題ではありません。**環境から安全に借用**することができます。

この例では、`v` はスポーン地点よりも前に作成されています。
これは `scope` が戻った*後*にのみドロップされます。同時に、
`scope` 内で生成されたすべてのスレッドは、`scope` が戻る*前*に完了することが保証されているため、
ダングリング参照が発生するリスクはありません。

コンパイラは警告を出しません！
