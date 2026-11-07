<a id="ticket-ids"></a>

# チケットID

チケット管理システムについて改めて考えてみましょう。\
現在のチケットモデルは以下のようになっています。

```rust
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```


ここで一つ欠けているのが、チケットを一意に識別するための**識別子**です。\
この識別子はチケットごとに一意である必要があります。これは、新しいチケットが作成される際に
自動的に生成することで保証できます。

<a id="refining-the-model"></a>

## モデルの洗練

IDはどこに格納すべきでしょうか？\
`Ticket` 構造体に新しいフィールドを追加することも考えられます。

```rust
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```


しかし、チケットを作成する前にはIDが分かりません。そのため、最初からそこに持たせることはできません。\
任意（optional）にする必要があります。

```rust
pub struct Ticket {
    pub id: Option<TicketId>,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```


これも理想的ではありません。チケットをストアから取得するたびに `None` のケースを処理しなければならず、
チケットが作成されればIDは必ず存在するはずであるにもかかわらずです。

最善の解決策は、2つの異なる型で表現される、2つの異なるチケット**状態**を設けることです。
すなわち、`TicketDraft` と `Ticket` です。

```rust
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```


`TicketDraft` は、まだ作成されていないチケットです。IDもステータスも持っていません。\
`Ticket` は、作成済みのチケットです。IDとステータスを持っています。\
`TicketDraft` と `Ticket` の各フィールドはそれぞれ独自の制約を持っているため、2つの型間でロジックを
重複させる必要はありません。
