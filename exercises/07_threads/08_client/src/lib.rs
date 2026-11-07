use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: クライアントの実装を具体化してください。
pub struct TicketStoreClient {}

impl TicketStoreClient {
    // 簡略化のため、すべてのエラーで遠慮なくパニックを起こしてください。
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        todo!()
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        todo!()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    todo!()
}

// 公開終了です！今後はライブラリの内部実装となります。
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // 送信者はもういないため、安全に処理を中断して
                // サーバーをシャットダウンできます。
                break;
            }
        }
    }
}
