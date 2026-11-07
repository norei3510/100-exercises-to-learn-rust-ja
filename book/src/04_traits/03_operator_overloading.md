<a id="operator-overloading"></a>

# 演算子のオーバーロード

トレイトとは何かについて基本的な理解が得られたところで、改めて**演算子のオーバーロード**について見ていきましょう。
演算子のオーバーロードとは、`+`、`-`、`*`、`/`、`==`、`!=`などの演算子に対して、独自の動作を定義できる機能のことです。

<a id="operators-are-traits"></a>

## 演算子はトレイトである

Rustでは、演算子はトレイトです。\
各演算子に対応するトレイトが存在し、その演算子の動作を定義しています。
自作の型でそのトレイトを実装することで、対応する演算子の使用が**可能**になります。

例えば、[`PartialEq`トレイト](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)は、
`==`および`!=`演算子の動作を定義しています：

```rust
// Rust標準ライブラリの`PartialEq`トレイト定義
// （現時点では*若干*簡略化されています）
pub trait PartialEq {
    // 必須メソッド
    //
    // `Self`はRustのキーワードで、
    // 「トレイトを実装している型」を表します。
    fn eq(&self, other: &Self) -> bool;

    // 提供されたメソッド
    fn ne(&self, other: &Self) -> bool { ... }
}
```


`x == y`と記述すると、コンパイラは`x`と`y`の型に対する`PartialEq`トレイトの実装を探し、
`x == y`を`x.eq(y)`に置き換えます。これはシンタックスシュガーです！

主要な演算子の対応関係は以下の通りです：

| 演算子                 | トレイト                                                                   |
| ------------------------ | ----------------------------------------------------------------------- |
| `+`                      | [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)               |
| `-`                      | [`Sub`](https://doc.rust-lang.org/std/ops/trait.Sub.html)               |
| `*`                      | [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html)               |
| `/`                      | [`Div`](https://doc.rust-lang.org/std/ops/trait.Div.html)               |
| `%`                      | [`Rem`](https://doc.rust-lang.org/std/ops/trait.Rem.html)               |
| `==` および `!=`            | [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)   |
| `<`、`>`、`<=`、および `>=` | [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) |

算術演算子は[`std::ops`](https://doc.rust-lang.org/std/ops/index.html)モジュールに、
比較演算子は[`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html)モジュールに定義されています。

<a id="default-implementations"></a>

## デフォルト実装

`PartialEq::ne`のコメントには「`ne`は提供されているメソッドである」と記載されています。\
これは、`PartialEq`がトレイト定義の中で`ne`の**デフォルト実装**を提供していることを意味します。定義スニペット内の`{ ... }`省略された
ブロックがそれに当たります。\
この省略されたブロックを展開すると、次のようになります：

```rust
pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
```


予想通り、`ne`は`eq`の否定です。\
デフォルト実装が提供されているため、自作の型で`PartialEq`を実装する際に`ne`の実装を省略できます。
`eq`を実装するだけで十分です：

```rust
struct WrappingU8 {
    inner: u8,
}

impl PartialEq for WrappingU8 {
    fn eq(&self, other: &WrappingU8) -> bool {
        self.inner == other.inner
    }
    
    // ここに`ne`の実装はありません
}
```


ただし、必ずしもデフォルト実装を使用する必要はありません。
トレイトを実装する際に、それをオーバーライドすることもできます：

```rust
struct MyType;

impl PartialEq for MyType {
    fn eq(&self, other: &MyType) -> bool {
        // 独自実装
    }

    fn ne(&self, other: &MyType) -> bool {
        // 独自実装
    }
}
```
