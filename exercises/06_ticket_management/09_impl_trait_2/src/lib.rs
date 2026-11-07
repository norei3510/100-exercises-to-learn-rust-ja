// TODO: `TicketStore::add_ticket` のシグネチャを、`impl Trait` 構文ではなく
// ジェネリック型パラメータを使用するように書き換えてください。

use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    // `ticket` の型パラメータに `Into<Ticket>` を指定すると、このメソッドは `Ticket` に確実に変換可能なあらゆる型を
    // 受け取れるようになります。
    // これにより、呼び出し側で `.into()` の構文上のノイズが排除され、メソッドの使い勝手が向上します。
    // ただし、コンパイラのエラーメッセージの質が低下する可能性があります。
    pub fn add_ticket(&mut self, ticket: impl Into<Ticket>) {
        self.tickets.push(ticket.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
    }

    impl From<TicketDraft> for Ticket {
        fn from(draft: TicketDraft) -> Self {
            Self {
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            }
        }
    }

    #[test]
    fn generic_add() {
        let mut store = TicketStore::new();
        // `add_ticket` が引数の位置で `impl Trait` の構文を使用すると、コンパイルできません。
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
    }
}
