<a id="thiserror"></a>

# `thiserror`

少し脱線してしまいましたね。ですが、必要な寄り道でした！\
それでは本題に戻りましょう。カスタムエラー型と `thiserror` についてです。

<a id="custom-error-types"></a>

## カスタムエラー型

カスタムエラー型に対して `Error` トレイトを「手動で」実装する方法を見てきました。\
もし、コードベース内のほとんどのエラー型でこれを行う必要があるとしたら、かなりのボイラープレートになりますよね。

[`thiserror`](https://docs.rs/thiserror/latest/thiserror/) を使えば、こうしたボイラープレートの一部を削減できます。
これは、カスタムエラー型の作成を簡素化する **手続き型マクロ** を提供する Rust クレートです。

```rust
#[derive(thiserror::Error, Debug)]
enum TicketNewError {
    #[error("{0}")]
    TitleError(String),
    #[error("{0}")]
    DescriptionError(String),
}
```


<a id="you-can-write-your-own-macros"></a>

## 独自のマクロを作成できる

これまで見てきた `derive` マクロはすべて Rust 標準ライブラリで提供されていたものです。\
`thiserror::Error` は、**サードパーティ製** `derive` マクロの最初の例です。

`derive` マクロは **手続き型マクロ** のサブセットであり、コンパイル時に Rust コードを生成するための仕組みです。
このコースでは手続き型マクロの書き方の詳細までは扱いませんが、
独自のマクロを作成できるということを知っておくことは重要です！\
これは、より高度な Rust コースで扱うトピックです。

<a id="custom-syntax"></a>

## カスタム構文

各手続き型マクロは独自の構文を定義でき、通常はクレートのドキュメントで説明されています。
`thiserror` の場合、以下のような構文があります。

- `#[derive(thiserror::Error)]`: これは `thiserror` の助けを借りて、カスタムエラー型に `Error` トレイトを派生させるための構文です。
- `#[error("{0}")]`: これはカスタムエラー型の各バリアントに対して `Display` の実装を定義するための構文です。
  `{0}` は、エラーが表示される際に、そのバリアントの 0 番目のフィールド（この場合は `String`）に置き換えられます。
