<a id="loops-part-1-while"></a>

# ループ（パート1）: `while`

`factorial` の実装では、再帰を使わざるを得ませんでした。\
関数型プログラミングの経験がある方には、これが自然に感じられるかもしれません。
あるいは、CやPythonのような命令型言語に慣れている方には、違和感があるかもしれません。

では、代わりに**ループ**を使って同じ機能を実装する方法を見ていきましょう。

<a id="the-while-loop"></a>

## `while` ループ

`while` ループは、**条件**が真である限り、コードブロックを実行する仕組みです。\
一般的な構文は以下の通りです。

```rust
while <condition> {
    // 実行するコード
}
```


例えば、1から5までの数値の合計を求めたいとします。

```rust
let sum = 0;
let i = 1;
// 「iが5以下の間」
while i <= 5 {
    // `+=`は`sum = sum + i`の略記です。
    sum += i;
    i += 1;
}
```


これにより、`i` が5以下ではなくなるまで、`i` に1を加算し、`i` を `sum` に加算し続けます。

<a id="the-mut-keyword"></a>

## `mut` キーワード

上記の例はそのままではコンパイルできません。次のようなエラーが発生します。

```text
error[E0384]: cannot assign twice to immutable variable `sum`
 --> src/main.rs:7:9
  |
2 |     let sum = 0;
  |         ---
  |         |
  |         first assignment to `sum`
  |         help: consider making this binding mutable: `mut sum`
...
7 |         sum += i;
  |         ^^^^^^^^ cannot assign twice to immutable variable

error[E0384]: cannot assign twice to immutable variable `i`
 --> src/main.rs:8:9
  |
3 |     let i = 1;
  |         -
  |         |
  |         first assignment to `i`
  |         help: consider making this binding mutable: `mut i`
...
8 |         i += 1;
  |         ^^^^^^ cannot assign twice to immutable variable
```


これは、Rustの変数がデフォルトで**不変**であるためです。\
一度値を代入すると、その値を変更することはできません。

値の変更を許可したい場合は、`mut` キーワードを使用して変数を**可変**として宣言する必要があります。

```rust
// `sum`と`i`は可変になりました！
let mut sum = 0;
let mut i = 1;

while i <= 5 {
    sum += i;
    i += 1;
}
```


これでエラーなくコンパイルし、実行できるようになります。

<a id="further-reading"></a>

## 関連資料

- [`while` ループのドキュメント](https://doc.rust-lang.org/std/keyword.while.html)
