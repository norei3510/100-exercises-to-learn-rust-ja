use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(todo!()),
}

// サーバースレッドを起動してシステムを開始します。
// 1つ以上のクライアントがサーバーとやり取りする際に使用できる
// `Sender`インスタンスを返します。
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: サーバータスクは**絶対に**停止してはいけません。
//  ループに入り、チャンネルにコマンドが現れるのを
// 待ち、それを実行し、次に次のコマンドを
// 待つようにしてください。
pub fn server(receiver: Receiver<Command>) {}
