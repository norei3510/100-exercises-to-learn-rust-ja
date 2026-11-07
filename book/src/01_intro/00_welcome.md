<a id="welcome"></a>

# ようこそ

**「Rustを学ぶための100の演習」**へようこそ！

このコースでは、演習を一つずつ解きながら、Rustのコアコンセプトを学んでいきます。\
Rustの構文、型システム、標準ライブラリ、そしてエコシステムについて学習します。

Rustに関する事前知識は一切不要ですが、少なくとも他のプログラミング言語を一つは習得していることを前提としています。\
また、システムプログラミングやメモリ管理に関する事前知識も不要です。これらのトピックは本コースの中で扱います。

つまり、ゼロからスタートします！\
無理なく進められる小さなステップを積み重ねて、Rustの知識を身につけていきましょう。\
コースを終える頃には約100個の演習を完了しており、中小規模のRustプロジェクトに自信を持って取り組めるようになります。

<a id="methodology"></a>

## 学習方法

このコースは「実践を通じて学ぶ」という原則に基づいています。\
インタラクティブでハンズオンな形式になるよう設計されています。

[Mainmatter](https://mainmatter.com/rust-consulting/)は、本コースを教室形式で4日間かけて提供するために開発しました。受講者はそれぞれのペースでレッスンを進め、経験豊富な講師がガイダンスを提供し、質問に答え、必要に応じてトピックを深掘りします。\
次回の講師付きセッションへの申し込みは、[公式サイト](https://ti.to/mainmatter/rust-from-scratch-jan-2025)から行えます。\
貴社向けにプライベートセッションの開催をご希望の場合は、[お問い合わせください](https://mainmatter.com/contact/)。

独学で受講することも可能ですが、行き詰まった際にサポートしてくれる友人やメンターを見つけることをお勧めします。すべての演習の解答は、GitHubリポジトリの[`solutions`ブランチ](https://github.com/mainmatter/100-exercises-to-learn-rust/tree/solutions)で確認できます。

<a id="formats"></a>

## フォーマット

コース教材は[ブラウザ上](https://rust-exercises.com/100-exercises/)で閲覧できるほか、オフラインで閲覧するために[PDFファイルとしてダウンロード](https://rust-exercises.com/100-exercises-to-learn-rust.pdf)することも可能です。\
教材の印刷版をご希望の方は、[Amazonでペーパーバック版をご購入](https://www.amazon.com/dp/B0DJ14KQQG/)ください。

<a id="structure"></a>

## 構成

画面の左側を見ると、コースがセクションに分かれていることがわかります。\
各セクションでは、Rust言語の新しいコンセプトや機能を紹介します。\
理解度を確認するため、各セクションには解くべき演習が用意されています。

演習は[付随するGitHubリポジトリ](https://github.com/mainmatter/100-exercises-to-learn-rust)にあります。\
コースを開始する前に、必ずリポジトリをローカルマシンにクローンしてください：

```bash
# GitHubでSSHキーを設定済みの場合
git clone git@github.com:mainmatter/100-exercises-to-learn-rust.git
# それ以外の場合は、HTTPS URLを使用してください：
#   https://github.com/mainmatter/100-exercises-to-learn-rust.git
```


また、進捗を簡単に追跡し、必要に応じてメインリポジトリから更新を取り込めるよう、ブランチで作業することをお勧めします：

```bash
cd 100-exercises-to-learn-rust
git checkout -b my-solutions
```


すべての演習は`exercises`フォルダにあります。\
各演習はRustパッケージとして構成されています。\
パッケージには、演習本体、実施内容の指示（`src/lib.rs`内）、および解答を自動的に検証するためのテストスイートが含まれています。

<a id="tools"></a>

### ツール

このコースを進めるには、以下が必要です：

- [**Rust**](https://www.rust-lang.org/tools/install)。
  システムに `rustup` が既にインストールされている場合は、`rustup update`（またはシステムへの Rust インストール方法に応じた適切なコマンド）を実行し、最新の安定版が動作していることを確認してください。
- _（任意ですが推奨）_ Rust の自動補完に対応した IDE。
  以下のいずれかを推奨します：
  - [RustRover](https://www.jetbrains.com/rust/);
  - [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) 拡張機能を導入した [Visual Studio Code](https://code.visualstudio.com)。

<a id="workshop-runner-wr"></a>

<a id="wr-the-workshop-runner"></a>

### ワークショップランナー、`wr`

解答を検証するため、コースの進行をガイドするツール「`wr` CLI」（「workshop runner」の略）も用意しています。
[公式サイト](https://mainmatter.github.io/rust-workshop-runner/) の手順に従って `wr` をインストールしてください。

`wr` のインストールが完了したら、新しいターミナルを開き、リポジトリのルートフォルダに移動してください。
`wr` コマンドを実行してコースを開始します：

```bash
wr
```


`wr` が現在の演習の解答を検証します。\
現在のセクションの演習を解くまで、次のセクションへ進まないでください。

> コースを進めるにつれて、解答を Git にコミットしておくことをお勧めします。
> そうすることで、進捗を簡単に追跡でき、必要に応じて既知の時点から「再開」することが可能になります。

コースをお楽しみください！

<a id="author"></a>

## 著者

本コースは、[Mainmatter](https://mainmatter.com/rust-consulting/) のプリンシパル・エンジニアリング・
コンサルタントである [Luca Palmieri](https://www.lpalmieri.com/) 氏によって執筆されました。\
Luca 氏は 2018 年から Rust に携わっており、当初は TrueLayer、その後は AWS で活動していました。\
Luca 氏は、Rust でバックエンドアプリケーションを構築する方法を学ぶための定番リソースである
『["Zero to Production in Rust"](https://zero2prod.com)』の著者です。\
また、[`cargo-chef`](https://github.com/LukeMathWalker/cargo-chef)、
[Pavex](https://pavex.dev)、[`wiremock`](https://github.com/LukeMathWalker/wiremock-rs) を含む、さまざまなオープンソースの Rust プロジェクトの著者およびメンテナーでもあります。

<a id="license"></a>

## ライセンス

Copyright © 2024- Mainmatter GmbH (https://mainmatter.com)、
[PolyForm-Noncommercial-1.0.0](https://polyformproject.org/licenses/noncommercial/1.0.0) の下で公開されています。
