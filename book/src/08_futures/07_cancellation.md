<a id="cancellation"></a>

# キャンセル

保留中のフューチャーがドロップされるとどうなるでしょうか？\
ランタイムはそのフューチャーをポーリングしなくなるため、それ以上の進展はなくなります。
言い換えれば、その実行は**キャンセル**されたことになります。

実際の開発では、タイムアウトを扱う際にこのような状況がよく発生します。
例えば：

```rust
use tokio::time::timeout;
use tokio::sync::oneshot;
use std::time::Duration;

async fn http_call() {
    // [...]
}

async fn run() {
    // futureを、10ミリ秒後に期限が切れるように設定した`Timeout`でラップしてください。
    let duration = Duration::from_millis(10);
    if let Err(_) = timeout(duration, http_call()).await {
        println!("Didn't receive a value within 10 ms");
    }
}
```


タイムアウトが切れると、`http_call` が返すフューチャーはキャンセルされます。
これが `http_call` の本体であると想定してみましょう：

```rust
use std::net::TcpStream;

async fn http_call() {
    let (stream, _) = TcpStream::connect(/* */).await.unwrap();
    let request: Vec<u8> = /* */;
    stream.write_all(&request).await.unwrap();
}
```


各 yield ポイントが**キャンセルポイント**となります。\
`http_call` はランタイムによってプリエンプトされないため、`.await` を介してエグゼキュータに制御を戻して初めて破棄されます。
これは再帰的に適用されます。例えば `stream.write_all(&request)` の実装には複数の yield ポイントがある可能性が高いです。`http_call` がキャンセルされる前に _部分的な_ リクエストを送信し、その結果接続が切断されてボディの送信が完了しない、という事態は十分に起こり得ます。

<a id="clean-up"></a>

## クリーンアップ

Rust のキャンセル機構は非常に強力です。呼び出し側は、タスク自体のいかなる協力も必要とせずに、進行中のタスクをキャンセルできます。\
同時に、これは非常に危険な側面もあります。操作を中断する前に、いくつかのクリーンアップタスクが確実に実行されるよう、**グレースフルなキャンセル**を行うことが望ましい場合があります。

例えば、SQL トランザクション用の架空の API を考えてみましょう：

```rust
async fn transfer_money(
    connection: SqlConnection,
    payer_id: u64,
    payee_id: u64,
    amount: u64
) -> Result<(), anyhow::Error> {
    let transaction = connection.begin_transaction().await?;
    update_balance(payer_id, amount, &transaction).await?;
    decrease_balance(payee_id, amount, &transaction).await?;
    transaction.commit().await?;
}
```


キャンセル時には、保留中のトランザクションを放置するのではなく、明示的にアボートさせるのが理想的です。
残念ながら、Rust にはこのような**非同期**なクリーンアップ操作のための万全なメカニズムは用意されていません。

最も一般的な戦略は、`Drop` トレイトを利用して必要なクリーンアップ作業をスケジュールすることです。具体的には以下のような方法があります：

- ランタイム上で新しいタスクをスポーンする
- チャネルにメッセージをエンキューする
- バックグラウンドスレッドをスポーンする

最適な選択肢は状況によって異なります。

<a id="cancelling-spawned-tasks"></a>

## スポーンしたタスクのキャンセル

`tokio::spawn` を使用してタスクをスポーンすると、そのタスクをドロップすることはできなくなります。
それはランタイムに属することになるからです。\
それでも、必要に応じてその `JoinHandle` を使用してキャンセルすることができます：

```rust
async fn run() {
    let handle = tokio::spawn(/* 何らかの非同期タスク */);
    // 生成したタスクをキャンセルします
    handle.abort();
}
```


<a id="further-reading"></a>

## 関連資料

- `tokio` の `select!` マクロを使用して 2 つの異なるフューチャーを「競わせる」場合は、細心の注意を払ってください。
  **キャンセル安全性**を確保できない限り、同じタスクをループでリトライするのは危険です。
  詳細は [`select!` のドキュメント](https://tokio.rs/tokio/tutorial/select) を参照してください。\
  2 つの非同期データストリーム（例えばソケットとチャネル）をインターリーブする必要がある場合は、代わりに [`StreamExt::merge`](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.merge) の使用を検討してください。
- 場合によっては、`JoinHandle::abort` よりも [`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) の方が適していることがあります。
