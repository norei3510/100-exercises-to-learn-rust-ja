//! TODO: `example` 関数内のステートメントを**並べ替える**ことで、
//! コードがコンパイルできるようにしてください。`spawner` 関数や、
//! `example` の各行の処理内容は変更してはいけません。
//!   必要に応じて、既存のステートメントをブロック `{}` で囲むことができます。
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
