<a id="spawning-tasks"></a>

# タスクの生成

前回の演習の解答は、おそらく次のような形になっているはずです：

```rust
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
        tokio::io::copy(&mut reader, &mut writer).await?;
    }
}
```


これは悪くない出来です！\
2つの接続の間に長い時間が空いた場合、`echo` 関数はアイドル状態になります
（`TcpListener::accept` は非同期関数であるため）。これにより、エグゼキュータは
その間に他のタスクを実行できるようになります。

では、実際に複数のタスクを並行して実行するにはどうすればよいでしょうか？\
もし非同期関数を常に完了まで実行し続ける（`.await` を使用する）なら、一度に
実行できるタスクは1つだけになってしまいます。

そこで `tokio::spawn` 関数の出番です。

<a id="tokiospawn"></a>

## `tokio::spawn`

`tokio::spawn` を使用すると、**完了を待たずに** タスクをエグゼキュータに渡すことができます。\
`tokio::spawn` を呼び出すたびに、`tokio` に対して、生成したタスクを
バックグラウンドで、それを生成したタスクと**並行して**実行し続けるよう指示することになります。

複数の接続を並行して処理するために、次のように利用できます：

```rust
use tokio::net::TcpListener;

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        // 接続処理を行うバックグラウンドタスクを起動します。
        // これにより、メインタスクは直ちに新しい接続の
        // 受け付けを開始できるようになります。
        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            tokio::io::copy(&mut reader, &mut writer).await?;
        });
    }
}
```


<a id="asynchronous-blocks"></a>

### 非同期ブロック

この例では、`tokio::spawn` に**非同期ブロック**を渡しています：`async move { /* */ }`
非同期ブロックは、別途 async 関数を定義することなく、コードの一部分を
非同期としてマークできる便利な方法です。

<a id="joinhandle"></a>

### `JoinHandle`

`tokio::spawn` は `JoinHandle` を返します。\
生成したスレッドで `join` を使用したときと同様に、`JoinHandle` を使って
バックグラウンドタスクを `.await` できます。

```rust
pub async fn run() {
    // テレメトリデータをリモートサーバーに送信する
    // バックグラウンドタスクを起動します。
    let handle = tokio::spawn(emit_telemetry());
    // その間、何か別の有益な作業をしてください。
    do_work().await;
    // ただし、テレメトリデータが正常に送信されるまで、
    // 呼び出し元に戻らないでください。
    handle.await;
}

pub async fn emit_telemetry() {
    // [...]
}

pub async fn do_work() {
    // [...]
}
```


<a id="panic-boundary"></a>

### パニック境界

`tokio::spawn` で生成したタスクがパニックを起こした場合、そのパニックはエグゼキュータによって捕捉されます。\
対応する `JoinHandle` を `.await` しなければ、パニックは生成元に伝播しません。
たとえ `JoinHandle` を `.await` したとしても、パニックは自動的に伝播しません。
`JoinHandle` を await すると `Result` が返され、そのエラー型は [`JoinError`](https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html)
となります。その後、`JoinError::is_panic` を呼び出してタスクがパニックを起こしたかを確認し、
パニックをどう処理するか（ログに記録する、無視する、あるいは伝播させる）を選択できます。

```rust
use tokio::task::JoinError;

pub async fn run() {
    let handle = tokio::spawn(work());
    if let Err(e) = handle.await {
        if let Ok(reason) = e.try_into_panic() {
            // タスクがパニックを起こしました。
            // パニックのアンワインドを再開し、
            // 現在のタスクへと伝播させます。
            panic::resume_unwind(reason);
        }
    }
}

pub async fn work() {
    // [...]
}
```


<a id="stdthreadspawn-vs-tokiospawn"></a>

### `std::thread::spawn` vs `tokio::spawn`

`tokio::spawn` は、`std::thread::spawn` の非同期版のようなものだと考えることができます。

重要な違いに注目してください。`std::thread::spawn` では、制御を OS スケジューラに委ねています。
スレッドがどのようにスケジューリングされるかは、自分で制御できません。

一方、`tokio::spawn` では、完全にユーザー空間で動作する非同期エグゼキュータに委ねています。
次にどのタスクを実行するかという決定に、基盤となる OS スケジューラは関与しません。
今回は、選択したエグゼキュータを通じて、私たちがその決定を制御することになります。
