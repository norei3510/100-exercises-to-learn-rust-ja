// TODO: このクレートの各モジュールで、やるべきことがあります！
mod description;
mod status;
mod title;

// Rustでよく見られるパターンは、コードを複数の（プライベート）モジュールに分割し、
// それらのモジュールの公開部分をクレートのルートで再エクスポートすることです。
//
// これにより、クレートの内部構造をユーザーから隠しつつ、
// コードを任意の方法で整理することができます。
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// フィールドをプライベートにする必要はもうありません！
// 各フィールドが独自の検証ロジックをカプセル化しているため、`Ticket`の利用者が
// 構造体の不変条件を損なう形でフィールドを変更する
// リスクはありません。
//
// ただしご注意ください。複数のフィールドにまたがる不変条件がある場合は、
// それらの不変条件が引き続き維持されるようにした上で、
// フィールドを再びprivateに戻す必要があります。
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
