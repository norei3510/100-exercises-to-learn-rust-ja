use std::sync::{Arc, Mutex, RwLock};
// TODO: 複数のリーダーがチケットストアに並行してアクセスできるよう、`TicketStore`構造体および
// その他すべての該当箇所の`Mutex`を`RwLock`に置き換えてください。
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadedError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Arc<RwLock<Ticket>>>, OverloadedError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: response_sender,
            })
            .map_err(|_| OverloadedError)?;
        Ok(response_receiver.recv().unwrap())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("The store is overloaded")]
pub struct OverloadedError;

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Arc<RwLock<Ticket>>>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
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
                let _ = response_channel.send(ticket);
            }
            Err(_) => {
                // 送信者はもういないため、安全に処理を中断して
                // サーバーをシャットダウンできます。
                break;
            }
        }
    }
}
