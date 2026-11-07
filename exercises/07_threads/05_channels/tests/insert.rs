// TODO: この演習が完了したと思ったら、`ready` 内の `move_forward` を `true` に書き換えてください。
//  解答を確認する際は、遠慮なく講師に声をかけてください！
use channels::data::TicketDraft;
use channels::{launch, Command};
use std::time::Duration;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    sender
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
        // スレッドが実行中でない場合、チャネルが閉じられるため、
        // パニックが発生します。
        .expect("Did you actually spawn a thread? The channel is closed!");
}

#[test]
fn ready() {
    // この演習では、サーバーが**読み取り**操作を公開していないため、
    // 自動的に確認できることがほとんどありません。
    // 挿入が実際に行われているか、また正しく行われているかを
    // 知る術がありません。
    let move_forward = false;

    assert!(move_forward);
}
