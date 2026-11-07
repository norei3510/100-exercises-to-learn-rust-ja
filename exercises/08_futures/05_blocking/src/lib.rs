// TODO: `echo` サーバーは非同期プリミティブを使用していません。
//  テストを実行すると、呼び出し元とサーバー間のデッドロックにより、
//  ハングアップすることが確認できるはずです。
//  `echo` 内で `spawn_blocking` を使用してこの問題を解決してください。
use std::io::{Read, Write};
use tokio::net::TcpListener;
use tokio::task::spawn_blocking;

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (socket, _) = listener.accept().await?;
        let mut socket = socket.into_std()?;
        spawn_blocking(move || -> Result<(), anyhow::Error> {
            socket.set_nonblocking(false)?;
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer)?;
            socket.write_all(&buffer)?;
            Ok(())
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (listener, addr) = bind_random().await;
        tokio::spawn(echo(listener));

        let requests = vec![
            "hello here we go with a long message",
            "world",
            "foo",
            "bar",
        ];
        let mut join_set = JoinSet::new();

        for request in requests {
            join_set.spawn(async move {
                let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                let (mut reader, mut writer) = socket.split();

                // リクエストを送信します
                writer.write_all(request.as_bytes()).await.unwrap();
                // ソケットの書き込み側を閉じます
                writer.shutdown().await.unwrap();

                // 回答を読んでください
                let mut buf = Vec::with_capacity(request.len());
                reader.read_to_end(&mut buf).await.unwrap();
                assert_eq!(&buf, request.as_bytes());
            });
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
