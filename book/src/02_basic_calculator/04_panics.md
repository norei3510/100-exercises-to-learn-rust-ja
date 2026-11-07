<a id="panics"></a>

# パニック

[「変数」セクション](02_variables.md)で作成した `speed` 関数に戻ってみましょう。
おそらく次のようなコードだったはずです。

```rust
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    let distance = end - start;
    distance / time_elapsed
}
```


鋭い方なら、ある問題に気づいたかもしれません[^one]。`time_elapsed` が 0 だった場合はどうなるでしょうか？

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=36e5ddbe3b3f741dfa9f74c956622bac) で試してみることができます！\
プログラムは以下のエラーメッセージとともに終了します。

```text
thread 'main' panicked at src/main.rs:3:5:
attempt to divide by zero
```


これは **パニック** と呼ばれます。\
パニックとは、プログラムの実行を継続できないほど重大な問題が発生したことを知らせる Rust の仕組みであり、**回復不能なエラー**[^catching] です。0 での除算はこの種のエラーに該当します。

<a id="the-panic-macro"></a>

## panic! マクロ

`panic!` マクロ[^macro]を呼び出すことで、意図的にパニックを発生させることができます。

```rust
fn main() {
    panic!("This is a panic!");
    // 以下の行が実行されることはありません
    let x = 1 + 2;
}
```


Rust には回復可能なエラーを扱うための他の仕組みもあり、それらについては[後ほど解説します](../05_ticket_v2/06_fallibility.md)。
当面は、強引ながらもシンプルな暫定策として、パニックを利用しましょう。

<a id="further-reading"></a>

## さらに詳しく

- [panic! マクロのドキュメント](https://doc.rust-lang.org/std/macro.panic.html)

[^one]: `speed` にはもう一つ問題がありますが、これについては後ほど取り上げます。お気づきでしょうか？

[^catching]: パニックをキャッチしようとすることもできますが、それは非常に特殊な状況に限定した最終手段とすべきです。

[^macro]: その後に `!` が続いている場合、それはマクロの呼び出しです。今のところ、マクロは「特殊な関数」のようなものだと考えてください。コースの後半でより詳しく解説します。
