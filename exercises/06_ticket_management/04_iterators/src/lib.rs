use ticket_fields::{TicketDescription, TicketTitle};

// TODO: チケットストアの設計に取り掛かりましょう！
//  最初のタスク：`TicketStore`に`IntoIterator`を実装し、`for`ループを用いて
//  格納されているすべてのチケットを反復処理できるようにします。
//
// ヒント：この場合、`Iterator` トレイトを実装する必要はないはずです。
//   イテレーションを `TicketStore` の `Vec<Ticket>` フィールドに*委譲*してください。
//   `into_iter` から返す適切な型を確認するには、標準ライブラリの `Vec` のドキュメントを参照してください。
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

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}
