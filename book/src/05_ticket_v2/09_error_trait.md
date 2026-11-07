<a id="error-trait"></a>

# Error トレイト

<a id="error-reporting"></a>

## エラー報告

前回の演習では、`TitleError` バリアントを分解してエラーメッセージを抽出し、
それを `panic!` マクロに渡す必要がありました。\
これは**エラー報告**の（初歩的な）例です。つまり、エラー型を、ユーザーやサービス運用者、あるいは開発者に提示できる形式に変換することです。

Rust の開発者がそれぞれ独自のエラー報告戦略を考案するのは現実的ではありません。時間の浪費になりますし、
プロジェクト間での組み合わせもうまくいきません。
そのため、Rust では `std::error::Error` トレイトが提供されています。

<a id="the-error-trait"></a>

## `Error` トレイト

`Result` における `Err` バリアントの型に制約はありませんが、`Error` トレイトを実装した型を使用するのが良いプラクティスです。
`Error` は Rust のエラーハンドリングにおける根幹となるものです：

```rust
// `Error` トレイトの定義を若干簡略化したもの
pub trait Error: Debug + Display {}
```


`:` の構文は `From` トレイト ([参照](../04_traits/09_from.md#supertrait--subtrait)) で見たことがあるかもしれません。これは**スーパートレイト**を指定するために使用されます。
`Error` には `Debug` と `Display` という2つのスーパートレイトがあります。ある型が `Error` を実装する場合、`Debug` と `Display` も実装しなければなりません。

<a id="display-and-debug"></a>

## `Display` と `Debug`

`Debug` トレイトについては [前回の演習] ([参照](../04_traits/04_derive.md)) ですでに触れました。これは `assert_eq!` が、アサーション失敗時に比較対象の変数の値を表示するために使用するトレイトです。

「機械的な」観点から見れば、`Display` と `Debug` は同一です。どちらも型を文字列のような表現にどのように変換すべきかを規定しています：

```rust
// `Debug`
pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

// `Display`
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```


違いはその*目的*にあります。`Display` は「エンドユーザー」向けの表現を返すのに対し、
`Debug` は開発者やサービス運用者に適した低レベルな表現を提供します。\
そのため、`Debug` は `#[derive(Debug)]` 属性を用いて自動的に実装できますが、`Display` は
手動での実装が**必要**となります。
