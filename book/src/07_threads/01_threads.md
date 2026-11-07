<a id="threads"></a>

# スレッド

マルチスレッドコードを書き始める前に、一度立ち返って、スレッドとは何か、
そしてなぜスレッドを使いたいのかについて説明しましょう。

<a id="what-is-a-thread"></a>

## スレッドとは？

**スレッド**とは、基盤となるオペレーティングシステムによって管理される実行コンテキストのことです。\
各スレッドは独自のスタックと命令ポインタを持っています。

単一の**プロセス**は、複数のスレッドを管理できます。
これらのスレッドは同じメモリ空間を共有するため、同じデータにアクセスできます。

スレッドは**論理的な**構成概念です。結局のところ、**物理的な**実行単位であるCPUコア上では、
一度に1つの命令セットしか実行できません。\
スレッドの数はCPUコアの数よりもはるかに多い場合があるため、オペレーティングシステムの
**スケジューラ**が、任意の時点でどのスレッドを実行するかを決定し、
CPU時間を各スレッドに割り当てることで、スループットと応答性を最大化します。

<a id="main"></a>

## `main`

Rustプログラムが起動すると、単一のスレッドである**メインスレッド**で実行されます。\
このスレッドはオペレーティングシステムによって作成され、`main`
関数を実行する役割を担います。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```


<a id="stdthread"></a>

## `std::thread`

Rustの標準ライブラリには、スレッドの作成と管理を可能にする`std::thread`というモジュールが用意されています。

<a id="spawn"></a>

### `spawn`

`std::thread::spawn`を使用して新しいスレッドを作成し、そこでコードを実行できます。

例えば：

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });
    
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```


このプログラムを[Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=afedf7062298ca8f5a248bc551062eaa)で実行すると、
メインスレッドと生成されたスレッドが並行して動作することがわかります。\
各スレッドは、互いに独立して処理を進めます。

<a id="process-termination"></a>

### プロセスの終了

メインスレッドが終了すると、プロセス全体が終了します。\
生成されたスレッドは、処理が完了するか、あるいはメインスレッドが終了するまで実行を続けます。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });

    thread::sleep(Duration::from_secs(5));
}
```


上記の例では、「Hello from a thread!」というメッセージがおよそ5回出力されることが予想されます。\
その後、メインスレッドが終了し（`sleep`の呼び出しが戻ったとき）、プロセス全体が終了するため、
生成されたスレッドも終了します。

<a id="join"></a>

### `join`

また、`spawn`が返す`JoinHandle`に対して`join`メソッドを呼び出すことで、生成されたスレッドの終了を待機することもできます。

```rust
use std::thread;
fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}
```


この例では、メインスレッドは生成されたスレッドが終了するまで待機してから終了します。\
これにより、2つのスレッド間に一種の**同期**が導入されます。生成されたスレッドが完了するまで
メインスレッドは終了しないため、プログラムが終了する前に「Hello from a thread!」というメッセージが
出力されることが保証されます。
