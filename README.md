<a id="learn-rust-one-exercise-at-a-time"></a>

# 演習を一つずつこなしてRustを学ぼう

> [!IMPORTANT]
> Mainmatter GmbH（原著者: Luca Palmieri）による「100 Exercises To Learn Rust」の**非公式AI翻訳版**です。
> 翻訳には大学GPU上の [PLaMo 3 Translate 31B](https://huggingface.co/pfnet/plamo-3-translate-31b-GGUF)（GGUF Q4_K_M）を使用しました。
> [原文リポジトリ](https://github.com/mainmatter/100-exercises-to-learn-rust) / [翻訳の記録](TRANSLATION.md)
> 原文のライセンスは [PolyForm Noncommercial 1.0.0](https://polyformproject.org/licenses/noncommercial/1.0.0) です。

Rustのことは聞いたことがあるけれど、実際に試す機会がなかったという方はいらっしゃいませんか？\
そんな方にぴったりのコースです！

100個の演習問題を解くことで、Rustを習得できます。\
Rustに関する知識が全くない状態から、演習を一つずつ進めることで、
自分でプログラムを書き始められるようになります。

> [!NOTE]
> 本コースは [Mainmatter](https://mainmatter.com/rust-consulting/) によって作成されました。\
> [Rustワークショップのポートフォリオ](https://mainmatter.com/services/workshops/rust/) に含まれるトレーニングの一つです。\
> Rustのコンサルティングやトレーニングをお探しの方は、\
> [ランディングページ](https://mainmatter.com/rust-consulting/) をご覧ください！

<a id="getting-started"></a>

## はじめに

[日本語版の教材サイト](https://norei3510.github.io/100-exercises-to-learn-rust-ja/) にアクセスし、記載されている手順に従って
コースを開始してください。本文をブラウザで読み、演習コードを手元で編集して `wr` または `cargo test` で確認する形式です。

[英語の原文サイト](https://rust-exercises.com/100-exercises/) も参照できます。

<a id="requirements"></a>

## 要件

- **Rust**（[こちら](https://www.rust-lang.org/tools/install) の手順に従ってください）。\
  システムに `rustup` が既にインストールされている場合は、`rustup update`（またはRustのインストール方法に応じた
  適切なコマンド）を実行し、
  最新の安定版が動作していることを確認してください。
- _（任意ですが推奨）_ Rustの自動補完に対応したIDE。
  以下のいずれかを推奨します：
  - [RustRover](https://www.jetbrains.com/rust/);
  - [Visual Studio Code](https://code.visualstudio.com) と
    [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) 拡張機能。

<a id="solutions"></a>

## 解答

演習問題の解答は、このリポジトリの
[`solutions`ブランチ](https://github.com/norei3510/100-exercises-to-learn-rust-ja/tree/solutions) で確認できます。

<a id="license"></a>

# ライセンス

Copyright © 2024- Mainmatter GmbH (https://mainmatter.com)、[PolyForm-Noncommercial-1.0.0](https://polyformproject.org/licenses/noncommercial/1.0.0) の下で
公開されています。

本コースを社内で利用される場合は、ライセンスについて [お問い合わせください](mailto:training@mainmatter.com)。
また、本コースを [講師主導のワークショップ](https://mainmatter.com/training/learn-rust-starting-from-scratch/) としても提供しています。

## 教材サイトの更新・ローカル確認

`book/src/` のMarkdownと目次 `book/src/SUMMARY.md` が教材サイトの元データです。
`main` への更新をGitHub Actionsが自動でビルドし、GitHub Pagesへ公開します。Pull Requestではビルドと内部リンクの確認だけを行います。

ローカルで確認するにはPython 3と [mdBook 0.5.4](https://github.com/rust-lang/mdBook/releases/tag/v0.5.4) を用意し、リポジトリのルートで実行してください。

```sh
mdbook build book
python helpers/check_book_links.py book/book
mdbook serve book --open
```

各章の日本語の演習リンクは `helpers/mdbook_exercise_linker.py` が生成します。
Web版はHTMLのみを生成するため、PDF用のフォント、Pandoc、TeXや外部サービスの秘密情報は不要です。
GitHub Pagesの公開元はリポジトリの **Settings → Pages → Source: GitHub Actions** です。
