<a id="syntax"></a>

# 構文

<div class="warning">

先走らないでください！\
このセクションを始める前に、前のセクションの演習を完了させてください。\
演習は [course GitHub's repository](https://github.com/mainmatter/100-exercises-to-learn-rust) 内の `exercises/01_intro/00_welcome` にあります。\
コースを開始し、解答を確認するには [`wr`](00_welcome.md#wr-the-workshop-runner) を使用してください。

</div>

前のタスクは演習と呼ぶほどのものではありませんでしたが、すでにかなりの量の Rust の**構文**に触れています。
前の演習で使用した Rust の構文のあらゆる詳細を網羅するわけではありません。
その代わりに、細部に囚われず先に進めるための_最低限必要な範囲_を解説します。\
一歩ずつ進めていきましょう！

<a id="comments"></a>

## コメント

単一行コメントには `//` を使用できます：

```rust
// これは単一行コメントです
// 続いて別の単一行コメントです
```


<a id="functions"></a>

## 関数

Rust の関数は `fn` キーワードを使用し、その後に関数名、入力パラメータ、および
戻り値の型を続けて定義します。
関数の本体は波括弧 `{}` で囲みます。

前の演習では、`greeting` 関数が登場しました：

```rust
// `fn` <function_name> ( <input params> ) -> <return_type> { <body> }
fn greeting() -> &'static str {
    // TODO: 修正してください 👇
    "I'm ready to __!"
}
```


`greeting` は入力パラメータを持たず、文字列スライス (`&'static str`) への参照を返します。

<a id="return-type"></a>

### 戻り値の型

関数が何も返さない場合（つまり Rust のユニット型である `()` を返す場合）、シグネチャから戻り値の型を省略できます。
`test_welcome` 関数がまさにその例です：

```rust
fn test_welcome() {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
```


上記は以下と等価です：

```rust
// ユニット戻り値の型を明示的に記述する
//                   👇
fn test_welcome() -> () {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
```


<a id="returning-values"></a>

### 値の返却

関数内の最後の式は暗黙的に返されます：

```rust
fn greeting() -> &'static str {
    // これは関数内の最後の式です。
    // したがって、その値が `greeting` によって返されます。
    "I'm ready to learn Rust!"
}
```


また、`return` キーワードを使用して、早期に値を返すこともできます：

```rust
fn greeting() -> &'static str {
    // 行末のセミコロンに注目してください！
    return "I'm ready to learn Rust!";
}
```


可能な限り `return` キーワードを省略するのが慣用的な書き方とされています。

<a id="input-parameters"></a>

### 入力パラメータ

入力パラメータは、関数名に続く括弧 `()` の中で宣言します。\
各パラメータは、名前、コロン `:`、そして型の順に宣言します。

例えば、以下の `greet` 関数は、`&str`（「文字列スライス」）型の `name` パラメータを受け取ります：

```rust
// 入力パラメータ
//        👇
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```


入力パラメータが複数ある場合は、カンマで区切る必要があります。

<a id="type-annotations"></a>

### 型アノテーション

「型」について何度か言及しましたので、明確に述べておきます。Rust は**静的型付け言語**です。\
Rust のあらゆる値には型があり、その型はコンパイル時にコンパイラに知られている必要があります。

型は**静的解析**の一種です。\
型とは、コンパイラがプログラム内のすべての値に付ける**タグ**のようなものだと考えることができます。タグに応じて、コンパイラは異なるルールを適用できます。例えば、文字列に数値を足すことはできませんが、2つの数値を足し合わせることは可能です。
型を正しく活用すれば、ランタイムバグのクラス全体を防ぐことができます。
