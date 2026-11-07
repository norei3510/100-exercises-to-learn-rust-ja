<a id="async-aware-primitives"></a>

# 非同期対応プリミティブ

`tokio` のドキュメントを参照すると、標準ライブラリにあるものを「ミラーリング」しつつ、非同期的な工夫を加えた型が数多く提供されていることに気づくでしょう。具体的には、ロック、チャネル、タイマーなどが挙げられます。

非同期コンテキストで開発を行う際は、同期版の型ではなく、これらの非同期対応版を優先して使用すべきです。

その理由を理解するために、前章で取り上げた相互排他ロックである `Mutex` を見てみましょう。

<a id="case-study-mutex"></a>

## ケーススタディ: `Mutex`

簡単な例を見てみましょう：

```rust
use std::sync::{Arc, Mutex};

async fn run(m: Arc<Mutex<Vec<u64>>>) {
    let guard = m.lock().unwrap();
    http_call(&guard).await;
    println!("Sent {:?} to the server", &guard);
    // `guard`はここで除外されます
}

/// HTTPリクエストのボディとして`v`を使用してください。
async fn http_call(v: &[u64]) {
  // [...]
}
```


<a id="stdsyncmutexguard-and-yield-points"></a>

### `std::sync::MutexGuard` と yield ポイント

このコードはコンパイルは通りますが、危険です。

非同期コンテキストにおいて、`std` から `Mutex` に対するロックを取得しようとしています。そして、得られた `MutexGuard` を yield ポイント（`http_call` 上の `.await`）をまたいで保持し続けています。

シングルスレッドのランタイム上で、2つのタスクが `run` を並行して実行している状況を想定してみましょう。次のようなスケジューリングイベントのシーケンスが発生します：

```text
     Task A          Task B
        | 
  Acquire lock
Yields to runtime
        | 
        +--------------+
                       |
             Tries to acquire lock
```


デッドロックが発生しました。タスク B はいつまで経ってもロックを取得できません。なぜなら、ロックは現在タスク A が保持しており、タスク A はロックを解放する前にランタイムに yield したためです。また、ランタイムはタスク B をプリエンプトできないため、タスク A は再びスケジューリングされることはありません。

<a id="tokiosyncmutex"></a>

### `tokio::sync::Mutex`

`tokio::sync::Mutex` に切り替えることで、この問題を解決できます：

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

async fn run(m: Arc<Mutex<Vec<u64>>>) {
    let guard = m.lock().await;
    http_call(&guard).await;
    println!("Sent {:?} to the server", &guard);
    // `guard`はここで除外されます
}
```


ロックの取得が非同期操作となり、処理が進まない場合はランタイムに yield するようになります。\
先ほどのシナリオに戻ると、次のようなことが起こります：

```text
       Task A          Task B
          | 
  Acquires the lock
  Starts `http_call`
  Yields to runtime
          | 
          +--------------+
                         |
             Tries to acquire the lock
              Cannot acquire the lock
                 Yields to runtime
                         |
          +--------------+
          |
`http_call` completes      
  Releases the lock
   Yield to runtime
          |
          +--------------+
                         |
                 Acquires the lock
                       [...]
```


これで問題ありません！

<a id="multithreaded-wont-save-you"></a>

### マルチスレッドでも解決しない

前の例では実行コンテキストとしてシングルスレッドのランタイムを使用しましたが、マルチスレッドのランタイムを使用した場合でも同様のリスクが存在します。\
唯一の違いは、デッドロックを発生させるために必要な並行タスクの数です。シングルスレッドのランタイムでは 2 つで十分ですが、マルチスレッドのランタイムでは `N+1` 個のタスクが必要になります。ここで `N` はランタイムのスレッド数です。

<a id="downsides"></a>

### デメリット

非同期対応の `Mutex` を使用すると、パフォーマンス上のペナルティが伴います。\
ロックに激しい競合が発生しないと確信しており、かつ yield ポイントをまたいでロックを保持しないよう注意しているのであれば、非同期コンテキストでも `std::sync::Mutex` を使い続けることができます。

ただし、パフォーマンス上のメリットと、それによって生じるライブネスのリスクを天秤にかけて検討してください。

<a id="other-primitives"></a>

## その他のプリミティブ

例として `Mutex` を挙げましたが、`RwLock` やセマフォなどにも同様のことが当てはまります。\
非同期コンテキストで開発を行う際は、問題発生のリスクを最小限に抑えるため、非同期対応版を優先して使用してください。
