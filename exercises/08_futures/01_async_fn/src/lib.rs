use tokio::net::TcpListener;

// TODO: 受信したTCP接続を受け付け、
// 受信したデータをクライアントに返送するエコーサーバーを実装してください。
//  `echo` は接続の処理が完了した際にリターンせず、
// 引き続き新しい接続を受け付けるようにしてください。
//
// ヒント：エコーサーバーの実装には、`tokio`の構造体とメソッドを活用してください。
// 具体的には：
// - `tokio::net::TcpListener::accept` 次に届く接続を処理するため
// - `tokio::net::TcpStream::split` ソケットからリーダーとライターを取得するため
// - `tokio::io::copy` リーダーからライターへデータをコピーするため
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
        tokio::io::copy(&mut reader, &mut writer).await?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello", "world", "foo", "bar"];

        for request in requests {
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
        }
    }
}
