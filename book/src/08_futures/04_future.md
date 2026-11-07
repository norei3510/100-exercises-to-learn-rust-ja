<a id="the-future-trait"></a>

# `Future` トレイト

<a id="the-local-rc-problem"></a>

## ローカルな `Rc` 問題

`tokio::spawn` のシグネチャに戻ってみましょう：

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
{ /* */ }
```


`F` が `Send` であるとは、_実際には_ どういう意味でしょうか？\
前節で見たように、これはスポーン元の環境からキャプチャした値が
`Send` でなければならないことを意味します。しかし、話はそれだけではありません。

_.await ポイントをまたいで保持される_ あらゆる値は `Send` である必要があります。\
例を見てみましょう：

```rust
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    // `Send`ではなく、
    // 非同期関数の_内部_で生成された値
    let non_send = Rc::new(1);
    
    // 何も行わない`.await`ポイント
    yield_now().await;

    // ローカルの非`Send`値は、
    // `.await`の後も引き続き必要です。
    println!("{}", non_send);
}
```


コンパイラはこのコードを拒絶します：

```text
error: future cannot be sent between threads safely
    |
5   |     tokio::spawn(example());
    |                  ^^^^^^^^^ 
    | future returned by `example` is not `Send`
    |
note: future is not `Send` as this value is used across an await
    |
11  |     let non_send = Rc::new(1);
    |         -------- has type `Rc<i32>` which is not `Send`
12  |     // A `.await` point
13  |     yield_now().await;
    |                 ^^^^^ 
    |   await occurs here, with `non_send` maybe used later
note: required by a bound in `tokio::spawn`
    |
164 |     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    |            ----- required by a bound in this function
165 |     where
166 |         F: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`
```


なぜそうなるのかを理解するには、Rust の非同期モデルについての理解を
深める必要があります。

<a id="the-future-trait-1"></a>

## `Future` トレイト

冒頭で述べたように、`async` 関数は **フューチャー**（`Future` トレイトを実装した型）を返します。フューチャーは **状態マシン** と考えることができます。
それは次の2つの状態のいずれかにあります：

- **pending**: 計算がまだ完了していません。
- **ready**: 計算が完了し、ここに出力があります。

これはトレイトの定義に組み込まれています：

```rust
trait Future {
    type Output;
    
    // `Pin`と`Context`は一旦無視してください
    fn poll(
      self: Pin<&mut Self>, 
      cx: &mut Context<'_>
    ) -> Poll<Self::Output>;
}
```


<a id="poll"></a>

### `poll`

`poll` メソッドは `Future` トレイトの中核です。\
フューチャー単体では何もしません。処理を進めるには **ポーリング** される必要があります。\
`poll` を呼び出すと、フューチャーに何らかの処理を行うよう要求することになります。
`poll` は処理を進めようとし、その後以下のいずれかを返します：

- `Poll::Pending`: フューチャーはまだ準備ができていません。後で再度 `poll` を呼び出す必要があります。
- `Poll::Ready(value)`: フューチャーが完了しました。`value` は計算結果であり、
  型は `Self::Output` です。

`Future::poll` が `Poll::Ready` を返した後は、再度ポーリングすべきではありません。フューチャーは
完了しており、やるべきことはもう何もないからです。

<a id="the-role-of-the-runtime"></a>

### ランタイムの役割

poll を直接呼び出すことは、あっても極めて稀でしょう。\
それは非同期ランタイムの役割です。ランタイムは、フューチャーが可能な限り処理を進められるよう、必要なすべての情報（`poll` のシグネチャにある `Context`）を保持しています。

<a id="async-fn-and-futures"></a>

## `async fn` とフューチャー

これまでは、高レベルインターフェースである非同期関数を扱ってきました。\
そして今、低レベルプリミティブである `Future trait` について見てきました。

これらはどのように関連しているのでしょうか？

関数を非同期としてマークするたびに、その関数はフューチャーを返します。
コンパイラは非同期関数の本体を **状態マシン** に変換します。
つまり、`.await` ポイントごとに1つの状態が割り当てられます。

`Rc` の例に戻ってみましょう：

```rust
use std::rc::Rc;
use tokio::task::yield_now;

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
```


コンパイラはこれを、おおよそ次のような列挙型に変換します：

```rust
pub enum ExampleFuture {
    NotStarted,
    YieldNow(Rc<i32>),
    Terminated,
}
```


`example` が呼び出されると、`ExampleFuture::NotStarted` を返します。フューチャーはまだ
一度もポーリングされていないため、何も起こっていません。\
ランタイムが最初にポーリングすると、`ExampleFuture` は次の
`.await` ポイントまで進みます。状態マシンの `ExampleFuture::YieldNow(Rc<i32>)` ステージで停止し、
`Poll::Pending` を返します。\
再度ポーリングされると、残りのコード (`println!`) を実行し、
`Poll::Ready(())` を返します。

その状態マシン表現 `ExampleFuture` を見ると、なぜ `example` が `Send` ではないのかが明確になります。`Rc` を保持しているため、
`Send` になることはあり得ないのです。

<a id="yield-points"></a>

## イールドポイント

`example` で見たように、すべての `.await` ポイントはフューチャーのライフサイクルにおいて新しい中間状態を作成します。\
そのため、`.await` ポイントは **イールドポイント** とも呼ばれます。フューチャーが制御を
ポーリングしていたランタイムに _戻す_ ことで、ランタイムは処理を一時停止し、（必要に応じて）別のタスクの実行をスケジュールでき、これにより複数の処理を並行して進めることができます。

イールドの重要性については、後のセクションで改めて説明します。
