<a id="static"></a>

# `'static`

前の演習でベクトルからスライスを借用しようとした場合、
おそらく次のようなコンパイラエラーが表示されたはずです。

```text
error[E0597]: `v` does not live long enough
   |
11 | pub fn sum(v: Vec<i32>) -> i32 {
   |            - binding `v` declared here
...
15 |     let right = &v[split_point..];
   |                  ^ borrowed value does not live long enough
16 |     let left_handle = spawn(move || left.iter().sum::<i32>());
   |                             -------------------------------- 
                     argument requires that `v` is borrowed for `'static`
19 | }
   |  - `v` dropped here while still borrowed
```


`argument requires that v is borrowed for 'static`、これはどういう意味でしょうか？

`'static` ライフタイムは、Rustにおける特殊なライフタイムです。\
これは、その値がプログラムの全期間にわたって有効であることを意味します。

<a id="detached-threads"></a>

## 切り離されたスレッド

`thread::spawn` を介して起動されたスレッドは、それを生成したスレッドよりも**長く生存**することがあります。\
例えば：

```rust
use std::thread;

fn f() {
    thread::spawn(|| {
        thread::spawn(|| {
            loop {
                thread::sleep(std::time::Duration::from_secs(1));
                println!("Hello from the detached thread!");
            }
        });
    });
}
```


この例では、最初に生成されたスレッドが、さらに
1秒ごとにメッセージを出力する子スレッドを生成します。\
その後、最初のスレッドは処理を完了して終了します。このとき、その
子スレッドはプロセス全体が動作している限り**実行され続けます**。\
Rustの用語では、子スレッドが親スレッドよりも**長く生存した**と言います。

<a id="static-lifetime"></a>

## `'static` ライフタイム

生成されたスレッドは、以下のことが可能であるため：

- それを生成したスレッド（親スレッド）よりも長く生存する
- プログラムが終了するまで実行される

プログラムが終了する前にドロップされる可能性のある値を借用してはなりません。\
この制約に違反すると、use-after-free バグを招くことになります。\
そのため、`std::thread::spawn` のシグネチャでは、渡されるクロージャが
`'static` ライフタイムを持つことが要求されています：

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
{
    // [..]
}
```


<a id="static-is-not-just-about-references"></a>

## `'static` は（単に）参照だけの話ではありません

Rustでは、参照だけでなく、すべての値がライフタイムを持ちます。

特に、データを所有する型（`Vec` や `String` など）は、
`'static` の制約を満たします。つまり、それを所有していれば、
元々それを作成した関数が返却された後でも、好きなだけ使い続けることができます。

したがって、`'static` は次のように解釈できます：

- 所有された値を渡せ
- プログラムの全期間にわたって有効な参照を渡せ

前者のアプローチは、前の演習で問題を解決した方法です。
元のベクトルの左側と右側を保持するための新しいベクトルを割り当て、
それらを生成されたスレッドに移動させました。

<a id="static-references"></a>

## `'static` 参照

次に、2つ目のケースである、プログラムの全期間にわたって有効な
参照について説明します。

<a id="static-data"></a>

### 静的データ

最も一般的なケースは、文字列リテラルなどの**静的データ**への参照です：

```rust
let s: &'static str = "Hello world!";
```


文字列リテラルはコンパイル時に判明するため、Rust はそれらを実行ファイルの
**読み取り専用データセグメント**と呼ばれる領域に格納します。
したがって、その領域を指すすべての参照は、プログラムが動作している限り
有効であり、`'static` の契約を満たします。

<a id="further-reading"></a>

## さらに詳しく

- [データセグメント](https://en.wikipedia.org/wiki/Data_segment)
