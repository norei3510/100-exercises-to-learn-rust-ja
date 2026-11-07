<a id="structs"></a>

# 構造体

各チケットについて、以下の3つの情報を管理する必要があります。

- タイトル
- 説明
- ステータス

まずは、これらを表現するために [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
を使用することから始めましょう。`String` は、Rust の標準ライブラリで定義されている、
[UTF-8 エンコード](https://en.wikipedia.org/wiki/UTF-8) されたテキストを表現するための型です。

では、これら3つの情報をどのようにして1つの実体に**まとめる**のでしょうか？

<a id="defining-a-struct"></a>

## `struct` の定義

`struct` は、**新しい Rust の型**を定義します。

```rust
struct Ticket {
    title: String,
    description: String,
    status: String
}
```


構造体は、他のプログラミング言語でいうクラスやオブジェクトと非常によく似ています。

<a id="defining-fields"></a>

## フィールドの定義

新しい型は、他の型を**フィールド**として組み合わせることで構築されます。\
各フィールドには名前と型が必要で、それらはコロンで区切られます `:`。フィールドが複数ある場合は、カンマで区切ります `,`。

以下の `Configuration` 構造体に見られるように、フィールドの型は必ずしも同じである必要はありません。

```rust
struct Configuration {
   version: u32,
   active: bool
}
```


<a id="instantiation"></a>

## インスタンス化

各フィールドの値を指定することで、構造体のインスタンスを作成できます。

```rust
// 構文: <StructName> { <field_name>: <value>, ... }
let ticket = Ticket {
    title: "Build a ticket system".into(),
    description: "A Kanban board".into(),
    status: "Open".into()
};
```


<a id="accessing-fields"></a>

## フィールドへのアクセス

`.` 演算子を使用して、構造体のフィールドにアクセスできます。

```rust
// フィールドアクセス
let x = ticket.description;
```


<a id="methods"></a>

## メソッド

**メソッド**を定義することで、構造体に振る舞いを持たせることができます。\
`Ticket` 構造体を例に説明します。

```rust
impl Ticket {
    fn is_open(self) -> bool {
        self.status == "Open"
    }
}

// 構文：
// impl <StructName> {
//    fn <method_name>(<parameters>) -> <return_type> {
//        // メソッド本体
//    }
// }
```


メソッドは関数と非常によく似ていますが、2つの重要な違いがあります。

1. メソッドは **`impl` ブロック** 内で定義する必要があります。
2. メソッドは第1引数として `self` を使用できます。
   `self` はキーワードであり、そのメソッドが呼び出されている構造体のインスタンスを表します。

<a id="self"></a>

### `self`

メソッドが第1引数として `self` を取る場合、**メソッド呼び出し構文**を使用して呼び出すことができます。

```rust
// メソッド呼び出しの構文: <instance>.<method_name>(<parameters>)
let is_open = ticket.is_open();
```


これは、[前の章](../02_basic_calculator/09_saturating.md) で `u32` の値に対して飽和算術演算を行った際と同じ呼び出し構文です。

<a id="static-methods"></a>

### 静的メソッド

メソッドが第1引数として `self` を取らない場合、それは**静的メソッド**となります。

```rust
struct Configuration {
    version: u32,
    active: bool
}

impl Configuration {
    // `default`は`Configuration`の静的メソッドです。
    fn default() -> Configuration {
        Configuration { version: 0, active: false }
    }
}
```


静的メソッドを呼び出す唯一の方法は、**関数呼び出し構文**を使用することです。

```rust
// 関数呼び出しの構文: <StructName>::<method_name>(<parameters>)
let default_config = Configuration::default();
```


<a id="equivalence"></a>

### 等価性

第1引数として `self` を取るメソッドであっても、関数呼び出し構文を使用できます。

```rust
// 関数呼び出しの構文：
//   <StructName>::<method_name>(<instance>, <parameters>)
let is_open = Ticket::is_open(ticket);
```


関数呼び出し構文を使えば、`ticket` がメソッドの第1引数である `self` として使用されていることが非常に明確になりますが、
間違いなく記述が冗長になります。可能な限りメソッド呼び出し構文を優先してください。
