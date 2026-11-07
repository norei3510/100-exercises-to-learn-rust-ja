<a id="enumerations"></a>

# 列挙型

[前の章](../03_ticket_v1/02_validation.md)で記述したバリデーションロジックに基づくと、
チケットの有効なステータスは `To-Do`、`InProgress`、`Done` のわずか数種類しかありません。\
`Ticket` 構造体の `status` フィールドや、`new` メソッドの `status`
パラメータの型を見ても、このことはすぐには分かりません。

```rust
#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: String
    ) -> Self {
        // [...]
    }
}
```


どちらの場合も、`status` フィールドを表すために `String` を使用しています。
`String` は非常に汎用的な型であり、`status` フィールドが取り得る値が
限定されているという情報が即座に伝わりません。さらに悪いことに、`Ticket::new` の呼び出し側は、指定したステータスが有効かどうかを**実行時**にしか
判断できません。

**列挙型**を使えば、より優れた実装が可能です。

<a id="enum"></a>

## `enum`

列挙型とは、**バリアント**と呼ばれる固定の値セットを持つことができる型のことです。\
Rustでは、`enum` キーワードを使用して列挙型を定義します。

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```


`enum` は、`struct` と同様に、**新しいRustの型**を定義します。
