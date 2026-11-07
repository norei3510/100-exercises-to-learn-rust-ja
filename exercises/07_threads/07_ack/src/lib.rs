use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

// 期待されるスキーマを理解するために、テストを参照してください。
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_sender: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_sender: Sender<Option<Ticket>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: 期待通りに受信したコマンドを処理してください。
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_sender,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_sender.send(id);
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                let ticket = store.get(id);
                let _ = response_sender.send(ticket.cloned());
            }
            Err(_) => {
                // 送信側（Sender）がもう存在しないため、安全に break して
                // サーバーをシャットダウンできます。
                break;
            }
        }
    }
}
