<a id="tryfrom-and-tryinto"></a>

# `TryFrom` と `TryInto`

前章では、**失敗のない**型変換のためのRustの慣用的なインターフェースである[`From` と `Into` トレイト](../04_traits/09_from.md)について見ました。\
では、変換の成功が保証されていない場合はどうすればよいでしょうか。

エラーについて十分に理解できたところで、`From` と `Into` に対応する**失敗しうる**トレイトである `TryFrom` と `TryInto` について解説します。

<a id="tryfrom-and-tryinto-1"></a>

## `TryFrom` と `TryInto`

`TryFrom` と `TryInto` は、`From` や `Into` と同様に、`std::convert` モジュールで定義されています。

```rust
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```


`From`/`Into` と `TryFrom`/`TryInto` の主な違いは、後者が `Result` 型を返す点です。\
これにより、パニックを起こす代わりにエラーを返して変換を失敗させることができます。

<a id="selferror"></a>

## `Self::Error`

`TryFrom` と `TryInto` には、それぞれ関連付けられた `Error` 型があります。
これにより、各実装で独自のエラー型を指定でき、理想的には試みられている変換に最も適切な型を使用できます。

`Self::Error` は、トレイト自体で定義された `Error` 関連型を参照する方法です。

<a id="duality"></a>

## 双対性

`From` と `Into` と同様に、`TryFrom` と `TryInto` は双対的なトレイトです。\
ある型に対して `TryFrom` を実装すれば、`TryInto` も自動的に得られます。
