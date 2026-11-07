<a id="trait-bounds"></a>

# トレイト境界

これまで、トレイトの2つのユースケースを見てきました：

- 「組み込み」の動作の有効化（例：演算子のオーバーロード）
- 既存の型への新しい動作の追加（すなわち拡張トレイト）

3つ目のユースケースがあります。それが**ジェネリックプログラミング**です。

<a id="the-problem"></a>

## 問題点

これまでの関数やメソッドは、すべて**具体的な型**を対象としていました。\
具体的な型を扱うコードは、通常、記述も理解も簡単です。しかし、再利用性には限界があります。\
例えば、整数が偶数である場合に `true` を返す関数を書きたいとしましょう。
具体的な型を扱う場合、サポートしたい整数型ごとに個別の関数を書く必要があります：

```rust
fn is_even_i32(n: i32) -> bool {
    n % 2 == 0
}

fn is_even_i64(n: i64) -> bool {
    n % 2 == 0
}

// など。
```


あるいは、単一の拡張トレイトを書き、整数型ごとに異なる実装を提供することもできます：

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// など。
```


それでも重複は残ります。

<a id="generic-programming"></a>

## ジェネリックプログラミング

**ジェネリクス**を使えば、より優れた方法があります。\
ジェネリクスを使用すると、具体的な型の代わりに**型パラメータ**を用いて動作するコードを記述できます：

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
{
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```


`print_if_even` は**ジェネリック関数**です。\
これは特定の入力型に紐付いていません。代わりに、以下の条件を満たす任意の型 `T` で動作します：

- `IsEven` トレイトを実装していること。
- `Debug` トレイトを実装していること。

この契約は**トレイト境界**を用いて表現されます：`T: IsEven + Debug`。\
`+` という記号は、`T` が複数のトレイトを実装していることを要求するために使用されます。`T: IsEven + Debug` は
「where `T` が `IsEven` **かつ** `Debug` を実装している」ことと同等です。

<a id="trait-bounds-1"></a>

## トレイト境界

`print_if_even` において、トレイト境界はどのような役割を果たすのでしょうか？\
それを確かめるために、トレイト境界を削除してみましょう：

```rust
fn print_if_even<T>(n: T) {
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```


このコードはコンパイルできません：

```text
error[E0599]: no method named `is_even` found for type parameter `T` 
              in the current scope
 --> src/lib.rs:2:10
  |
1 | fn print_if_even<T>(n: T) {
  |                  - method `is_even` not found 
  |                    for this type parameter
2 |     if n.is_even() {
  |          ^^^^^^^ method not found in `T`

error[E0277]: `T` doesn't implement `Debug`
 --> src/lib.rs:3:19
  |
3 |         println!("{n:?} is even");
  |                   ^^^^^ 
  |   `T` cannot be formatted using `{:?}` because 
  |         it doesn't implement `Debug`
  |
help: consider restricting type parameter `T`
  |
1 | fn print_if_even<T: std::fmt::Debug>(n: T) {
  |                   +++++++++++++++++
```


トレイト境界がないと、コンパイラは `T` が**何ができるのか**を把握できません。\
`T` に `is_even` メソッドがあることも、`T` を出力用にフォーマットする方法も分かりません。
コンパイラの視点からすると、単なる `T` には動作が全くないことになります。\
トレイト境界は、関数本体で要求される動作が存在することを保証することで、使用できる型の集合を制限します。

<a id="syntax-inlining-trait-bounds"></a>

## 構文：トレイト境界のインライン記述

上記の例ではすべて、**`where` 句**を使用してトレイト境界を指定していました：

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
//  ^^^^^^^^^^^^^^^^^
//  これは`where`条項です
{
    // [...]
}
```


トレイト境界が単純な場合は、型パラメータのすぐ隣に**インラインで**記述することもできます：

```rust
fn print_if_even<T: IsEven + Debug>(n: T) {
    //           ^^^^^^^^^^^^^^^^^
    //           これはインラインのトレイト境界です
    // [...]
}
```


<a id="syntax-meaningful-names"></a>

## 構文：意味のある名前

上記の例では、型パラメータの名前として `T` を使用しました。これは、関数に型パラメータが1つしかない場合の一般的な慣習です。\
とはいえ、より意味のある名前を使用することを妨げるものはありません：

```rust
fn print_if_even<Number: IsEven + Debug>(n: Number) {
    // [...]
}
```


実際には、複数の型パラメータが関わる場合や、`T` という名前では関数内での型の役割について十分な情報が伝わらない場合には、意味のある名前を使用することが**望ましい**です。
変数や関数パラメータと同様に、型パラメータの命名においても明確さと可読性を最大限に高めてください。
ただし、Rustの慣習に従い、[型パラメータ名にはアッパーキャメルケース](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case) を使用してください。

<a id="the-function-signature-is-king"></a>

## 関数シグネチャこそがすべて

そもそもなぜトレイト境界が必要なのか、疑問に思うかもしれません。コンパイラが関数本体から必要なトレイトを推論できるのではないでしょうか？\
可能ではありますが、実際には行いません。\
その理由は[関数パラメータへの明示的な型アノテーション](../02_basic_calculator/02_variables.md#function-arguments-are-variables)の場合と同様です。
各関数シグネチャは呼び出し側と呼び出し先との間の契約であり、その条件は明示的に記述される必要があります。
これにより、より優れたエラーメッセージ、より充実したドキュメント、バージョン間における意図しない不具合の減少、
そしてコンパイル時間の短縮が実現します。
