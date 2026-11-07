<a id="destructors"></a>

# デストラクタ

ヒープについて説明した際、確保したメモリの解放は開発者の責任であると述べました。\
また、借用チェッカーについて説明した際、Rustではメモリを直接管理する必要はほとんどないとも述べました。

これら2つの記述は、一見すると矛盾しているように思えるかもしれません。
**スコープ**と**デストラクタ**を導入して、これらがどのように整合するのか見ていきましょう。

<a id="scopes"></a>

## スコープ

変数の**スコープ**とは、その変数が有効である、つまり**生存している**Rustコードの領域のことです。

変数のスコープは、その宣言から始まります。
そして、以下のいずれかが発生したときに終了します。

1. 変数が宣言されたブロック（すなわち `{}` で囲まれたコード）が終了したとき
   ```rust
   fn main() {
      // `x` はここではまだスコープ外です
      let y = "Hello".to_string();
      let x = "World".to_string(); // <-- x のスコープはここから始まります...
      let h = "!".to_string(); //   |
   } //  <-------------- ...そしてここで終了します
   ```
2. 変数の所有権が他のもの（関数や他の変数など）に譲渡されたとき
   ```rust
   fn compute(t: String) {
      // 何らかの処理 [...]
   }

   fn main() {
       let s = "Hello".to_string(); // <-- s のスコープはここから始まります...
                   //                    | 
       compute(s); // <------------------- ..そしてここで終了します
                   //   `s` が `compute` に移動されるためです
   }
   ```

<a id="destructors-1"></a>

## デストラクタ

値の所有者がスコープ外に出ると、Rustはその**デストラクタ**を呼び出します。\
デストラクタは、その値が使用していたリソース、特に確保したメモリのクリーンアップを試みます。

`std::mem::drop` に値を渡すことで、その値のデストラクタを手動で呼び出すことができます。\
そのため、Rust開発者が値について「**ドロップされた**」と言うのをよく耳にするでしょう。これは、値がスコープ外に出てデストラクタが呼び出されたことを表す表現です。

<a id="visualizing-drop-points"></a>

### ドロップポイントの可視化

`drop` への明示的な呼び出しを挿入することで、コンパイラが自動で行っている処理を「具体的に記述」することができます。先ほどの例に戻りましょう。

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
}
```


これは以下と等価です。

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
   // 変数は宣言と逆の順序でドロップされます
   drop(h);
   drop(x);
   drop(y);
}
```


次に、`s` の所有権が `compute` に譲渡される2番目の例を見てみましょう。

```rust
fn compute(s: String) {
   // 何かしてください [...]
}

fn main() {
   let s = "Hello".to_string();
   compute(s);
}
```


これは以下と等価です。

```rust
fn compute(t: String) {
    // 何かしてください [...]
    drop(t); // <-- `t` が削除または移動されていないことを前提としています
             //     この時点より前に、コンパイラは
             //     スコープ外に出た際に、ここで `drop` を呼び出します。
}

fn main() {
    let s = "Hello".to_string();
    compute(s);
}
```


違いに注目してください。`main` で `compute` が呼び出された後、`s` は有効ではなくなりますが、`main` に `drop(s)` は存在しません。
値の所有権を関数に譲渡するとき、同時に**クリーンアップの責任も譲渡している**ことになります。

これにより、値のデストラクタが**最大で[^leak]1回**だけ呼び出されることが保証され、設計上 [double free bugs](https://owasp.org/www-community/vulnerabilities/Doubly_freeing_memory) を防ぐことができます。

<a id="use-after-drop"></a>

### ドロップ後の使用

値がドロップされた後にその値を使用しようとするとどうなるでしょうか。

```rust
let x = "Hello".to_string();
drop(x);
println!("{}", x);
```


このコードをコンパイルしようとすると、エラーが発生します。

```rust
error[E0382]: use of moved value: `x`
 --> src/main.rs:4:20
  |
3 |     drop(x);
  |          - value moved here
4 |     println!("{}", x);
  |                    ^ value used here after move
```


ドロップは、呼び出された値を**消費**します。つまり、呼び出し後はその値はもはや有効ではありません。\
したがって、コンパイラはその値の使用を禁止し、[use-after-free bugs](https://owasp.org/www-community/vulnerabilities/Using_freed_memory) を回避します。

<a id="dropping-references"></a>

### 参照のドロップ

変数が参照を含んでいる場合はどうなるでしょうか。\
例えば次のようになります。

```rust
let x = 42i32;
let y = &x;
drop(y);
```


`drop(y)` を呼び出しても... 何も起こりません。\
実際にこのコードをコンパイルしようとすると、警告が表示されます。

```text
warning: calls to `std::mem::drop` with a reference 
         instead of an owned value does nothing
 --> src/main.rs:4:5
  |
4 |     drop(y);
  |     ^^^^^-^
  |          |
  |          argument has type `&i32`
  |
```


これは先ほど述べた話に戻ります。デストラクタは一度だけ呼び出したいということです。\
同じ値に対して複数の参照を持つことができます。もしそのうちの1つがスコープ外になった際に、
それが指している値のデストラクタを呼び出したとしたら、他の参照はどうなるでしょうか？
それらはもはや有効ではないメモリ位置を参照することになります。いわゆる[**ダングリングポインタ**](https://en.wikipedia.org/wiki/Dangling_pointer)であり、
[**use-after-freeバグ**](https://owasp.org/www-community/vulnerabilities/Using_freed_memory)の近縁種です。
Rustの所有権システムは、設計上こうしたバグを排除しています。

[^leak]: Rustはデストラクタの実行を保証しません。例えば、
明示的に[メモリリーク](../07_threads/03_leak.md)させた場合などは、実行されません。
