<a id="control-flow-part-1"></a>

# 制御フロー（パート1）

これまでのプログラムはどれも非常にシンプルなものでした。\
一連の命令が上から下へと実行され、それで終わりです。

いよいよ**分岐**を導入しましょう。

<a id="if-clauses"></a>

## `if` 節

`if` キーワードは、条件が真である場合にのみコードブロックを実行するために使用されます。

簡単な例を挙げます：

```rust
let number = 3;
if number < 5 {
    println!("`number` is smaller than 5");
}
```


このプログラムは、条件 `number < 5` が真であるため、`number is smaller than 5` を出力します。

<a id="else-clauses"></a>

### `else` 節

多くのプログラミング言語と同様に、Rustでは `if` 式の条件が偽である場合にコードブロックを実行するためのオプションの `else` 分岐をサポートしています。\
例えば：

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else {
    println!("`number` is greater than or equal to 5");
}
```


<a id="else-if-clauses"></a>

### `else if` 節

`if` 式をネストして複数使用すると、コードがどんどん右にずれていきます。

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else {
    if number < 8 {
        println!("`number` is greater than or equal to 5, but smaller than 8");
    } else {
        println!("`number` is greater than or equal to 8");
    }
}
```


`else if` キーワードを使用すると、複数の `if` 式を一つにまとめることができます：

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else if number < 8 {
    println!("`number` is greater than or equal to 5, but smaller than 8");
} else {
    println!("`number` is greater than or equal to 8");
}
```


<a id="booleans"></a>

## ブール値

`if` 式の条件は、`bool` 型、すなわち**ブール値**である必要があります。\
ブール値は、整数と同様にRustのプリミティブ型です。

ブール値は、`true` または `false` のいずれかの値を取ります。

<a id="no-truthy-or-falsy-values"></a>

### truthy や falsy な値は存在しない

`if` 式の条件がブール値でない場合、コンパイルエラーが発生します。

例えば、以下のコードはコンパイルできません：

```rust
let number = 3;
if number {
    println!("`number` is not zero");
}
```


次のようなコンパイルエラーが発生します：

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if number {
  |        ^^^^^^ expected `bool`, found integer
```


これは型強制に関するRustの設計思想によるものです。非ブール型からブール型への自動変換は行われません。
Rustには、JavaScriptやPythonのような**truthy**や**falsy**な値という概念はありません。\
チェックしたい条件は明示的に記述する必要があります。

<a id="comparison-operators"></a>

### 比較演算子

`if` 式の条件を構築するために、比較演算子を使用することは非常に一般的です。\
整数を扱う際にRustで利用できる比較演算子は以下の通りです：

- `==`: 等しい
- `!=`: 等しくない
- `<`: 未満
- `>`: より大きい
- `<=`: 以下
- `>=`: 以上

<a id="ifelse-is-an-expression"></a>

## `if/else` は式である

Rustにおいて、`if` 式は文ではなく**式**であり、値を返します。\
その値は変数に代入したり、他の式で使用したりできます。例えば：

```rust
let number = 3;
let message = if number < 5 {
    "smaller than 5"
} else {
    "greater than or equal to 5"
};
```


上記の例では、`if` の各分岐が文字列リテラルとして評価され、\
それが `message` 変数に代入されます。\
唯一の要件は、両方の `if` 分岐が同じ型を返すことです。
