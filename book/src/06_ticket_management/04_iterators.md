<a id="iteration"></a>

# 反復処理

最初の演習で、Rustでは `for` ループを使ってコレクションを反復処理できることを学びました。
当時は範囲（例：`0..5`）を扱っていましたが、これは配列やベクタなどのコレクションでも同様です。

```rust
// `Vec`sで動作します。
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}

// 配列でも動作します
let a: [u32; 3] = [1, 2, 3];
for n in a {
    println!("{}", n);
}
```


ここからは、その仕組みの内部的な動作について理解していきましょう。

<a id="for-desugaring"></a>

## `for` デシュガーリング

Rustで `for` ループを記述するたびに、コンパイラはそれを以下のコードに _デシュガーリング_ します。

```rust
let mut iter = IntoIterator::into_iter(v);
loop {
    match iter.next() {
        Some(n) => {
            println!("{}", n);
        }
        None => break,
    }
}
```


`loop` は、`for` と `while` の上に構築されたもう一つのループ構文です。\
`loop` ブロックは、明示的に `break` で抜けない限り、永遠に実行され続けます。

<a id="iterator-trait"></a>

## `Iterator` トレイト

先ほどのコードスニペットにある `next` メソッドは、`Iterator` トレイトに由来します。
`Iterator` トレイトはRustの標準ライブラリで定義されており、値のシーケンスを生成できる型に
共通のインターフェースを提供します。

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```


`Item` 関連型は、イテレータが生成する値の型を指定します。

`next` は、シーケンスの次の値を返します。\
返す値がある場合は `Some(value)` を、ない場合は `None` を返します。

注意してください。イテレータが `None` を返したときに、必ずしも反復処理が終了しているとは限りません。それが保証されるのは、
イテレータが（より制約の厳しい）
[`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) トレイトを実装している場合のみです。

<a id="intoiterator-trait"></a>

## `IntoIterator` トレイト

すべての型が `Iterator` を実装しているわけではありませんが、多くは実装済みの型に変換可能です。\
そこで `IntoIterator` トレイトの出番となります。

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```


`into_iter` メソッドは元の値を消費し、その要素に対するイテレータを返します。\
ある型が持つ `IntoIterator` の実装は一つだけです。これにより、`for` が何にデシュガーリングされるべきかについて曖昧さがなくなります。

一点補足すると、`Iterator` を実装するすべての型は、自動的に `IntoIterator` も実装することになります。
`into_iter` から単に自分自身を返すだけなのです！

<a id="bounds-checks"></a>

## 境界チェック

イテレータによる反復処理には、設計上、境界外アクセスが発生しないという嬉しい副次効果があります。\
これにより、Rustは生成されるマシンコードから境界チェックを削除でき、反復処理を高速化できます。

言い換えれば、

```rust
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}
```


は通常、

```rust
let v = vec![1, 2, 3];
for i in 0..v.len() {
    println!("{}", v[i]);
}
```


よりも高速です。

このルールには例外もあります。コンパイラが、手動のインデックス指定であっても境界外アクセスが発生しないことを証明できれば、
同様に境界チェックを削除することがあります。しかし一般的には、可能な限りインデックス指定よりも反復処理を優先してください。
