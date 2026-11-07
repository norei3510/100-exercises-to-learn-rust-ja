<a id="from-and-into"></a>

# `From` と `Into`

文字列の学習を始めたところに戻ってみましょう：

```rust
let ticket = Ticket::new(
    "A title".into(), 
    "A description".into(), 
    "To-Do".into()
);
```


これで、ここで `.into()` が何をしているのかを解き明かし始めるのに十分な知識が身につきました。

<a id="the-problem"></a>

## 問題

これは `new` メソッドのシグネチャです：

```rust
impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: String
    ) -> Self {
        // [...]
    }
}
```


また、文字列リテラル（`"A title"` など）は `&str` 型であることも確認しました。\
ここで型の不一致が発生しています。期待されているのは `String` ですが、実際には `&str` となっています。
今回は魔法のような強制変換で解決することはできません。**変換を行う**必要があります。

<a id="from-and-into-1"></a>

## `From` と `Into`

Rust標準ライブラリの `std::convert` モジュールには、**失敗のない変換**のための2つのトレイト、`From` と `Into` が定義されています。

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}
```


これらのトレイト定義では、まだ見たことのないいくつかの概念が登場します。それが **スーパートレイト** と **暗黙的なトレイト境界** です。
まずはこれらを解き明かしていきましょう。

<a id="supertrait--subtrait"></a>

### スーパートレイト / サブトレイト

`From: Sized` という構文は、`From` が `Sized` の **サブトレイト** であることを意味します。つまり、`From` を実装するあらゆる型は、`Sized` も実装しなければなりません。
言い換えれば、`Sized` は `From` の **スーパートレイト** であると言えます。

<a id="implicit-trait-bounds"></a>

### 暗黙的なトレイト境界

ジェネリックな型パラメータを使用するたびに、コンパイラはそれが `Sized` であることを暗黙的に想定します。

例えば：

```rust
pub struct Foo<T> {
    inner: T,
}
```


は、実際には以下と等価です：

```rust
pub struct Foo<T: Sized> 
{
    inner: T,
}
```


`From<T>` の場合、トレイト定義は以下と等価になります：

```rust
pub trait From<T: Sized>: Sized {
    fn from(value: T) -> Self;
}
```


言い換えれば、前者の境界は暗黙的であるものの、`T` と `From<T>` を実装する型の _両方_ が `Sized` である必要があります。

<a id="negative-trait-bounds"></a>

### 否定的なトレイト境界

**否定的なトレイト境界** を使うことで、暗黙的な `Sized` 境界を除外できます：

```rust
pub struct Foo<T: ?Sized> {
    //            ^^^^^^^
    //            これは否定的なトレイト境界です
    inner: T,
}
```


この構文は「`T` は `Sized` であるかもしれないし、そうでないかもしれない」という意味であり、これにより `T` を DST（例：`Foo<str>`）にバインドできるようになります。ただし、これは特殊なケースです。否定的なトレイト境界は `Sized` 専用であり、他のトレイトでは使用できません。

<a id="str-to-string"></a>

## `&str` から `String`

[`std` のドキュメント](https://doc.rust-lang.org/std/convert/trait.From.html#implementors) では、どの `std` 型が `From` トレイトを実装しているかを確認できます。\
`String` が `From<&str> for String` を実装していることがわかります。したがって、次のように記述できます：

```rust
let title = String::from("A title");
```


しかし、これまで主に使用してきたのは `.into()` でした。\
`Into` の実装者一覧 ([参照](https://doc.rust-lang.org/std/convert/trait.Into.html#implementors)) を確認しても、`Into<String> for &str` は見当たりません。一体どういうことでしょうか？

`From` と `Into` は **デュアルトレイト** です。\
具体的には、`Into` は **ブランケット実装** を用いて、`From` を実装するあらゆる型に対して実装されています：

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```


型 `U` が `From<T>` を実装していれば、`Into<U> for T` は自動的に実装されます。だからこそ、`let title = "A title".into();` と記述できるのです。

<a id="into"></a>

## `.into()`

`.into()` を目にするたびに、それは型間の変換が行われていることを意味します。\
では、変換先の型は何でしょうか？

ほとんどの場合、変換先の型は以下のいずれかです：

- 関数やメソッドのシグネチャで指定されている（例：上記の例の `Ticket::new`）
- 型アノテーションを用いて変数宣言で指定されている（例：`let title: String = "A title".into();`）

`.into()` は、コンパイラがコンテキストから変換先の型を曖昧さなく推論できる限り、そのまま動作します。
