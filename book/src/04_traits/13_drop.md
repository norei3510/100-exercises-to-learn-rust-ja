<a id="the-drop-trait"></a>

# `Drop` トレイト

[デストラクタ](../03_ticket_v1/11_destructor.md) を紹介した際、
`drop` 関数が以下の処理を行うことに触れました。

1. 型が占有するメモリ（すなわち `std::mem::size_of` バイト）を回収する
2. 値が管理している可能性のある追加リソース（例：`String` のヒープバッファ）をクリーンアップする

ステップ 2 で登場するのが `Drop` トレイトです。

```rust
pub trait Drop {
    fn drop(&mut self);
}
```


`Drop` トレイトは、コンパイラが自動的に行う処理に加えて、
型に対する_追加の_クリーンアップロジックを定義するための仕組みです。\
`drop` メソッドに記述した内容は、値がスコープ外になった際に実行されます。

<a id="drop-and-copy"></a>

## `Drop` と `Copy`

`Copy` トレイトについて説明した際、型がメモリ上で占有する `std::mem::size_of` バイト以外の追加リソースを管理している場合、
その型は `Copy` を実装できないと述べました。

ここで、「コンパイラはどうやって型が追加リソースを管理しているか判断するのだろうか」と疑問に思うかもしれません。
その通りです。`Drop` トレイトの実装です！\
型に明示的な `Drop` 実装がある場合、コンパイラはその型に追加リソースが紐付いていると判断し、
`Copy` の実装を許可しません。

```rust
// これはユニット構造体、つまりフィールドを持たない構造体です。
#[derive(Clone, Copy)]
struct MyType;

impl Drop for MyType {
    fn drop(&mut self) {
       // ここでは特に何もする必要はありません。
       // 「空の」Drop実装があれば十分です。
    }
}
```


コンパイラは以下のエラーメッセージを表示します。

```text
error[E0184]: the trait `Copy` cannot be implemented for this type; 
              the type has a destructor
 --> src/lib.rs:2:17
  |
2 | #[derive(Clone, Copy)]
  |                 ^^^^ `Copy` not allowed on types with destructors
```
