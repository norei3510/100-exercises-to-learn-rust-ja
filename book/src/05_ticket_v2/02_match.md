<a id="match"></a>

# `match`

「列挙型で実際に**何ができる**のか」と疑問に思うかもしれません。\
最も一般的な操作は、列挙型を**マッチ**させることです。

```rust
enum Status {
    ToDo,
    InProgress,
    Done
}

impl Status {
    fn is_done(&self) -> bool {
        match self {
            Status::Done => true,
            // `|` 演算子を使うと、複数のパターンにマッチさせることができます。
            // 「`Status::ToDo` または `Status::InProgress`」という意味になります。
            Status::InProgress | Status::ToDo => false
        }
    }
}
```


Rustの値を一連の**パターン**と照合できる`match`文です。\
型レベルの`if`のようなものだと考えることができます。`status`が`Done`バリアントであれば最初のブロックを実行し、
`InProgress`または`ToDo`バリアントであれば2番目のブロックを実行します。

<a id="exhaustiveness"></a>

## 網羅性

ここで重要な点が一つあります。`match`は**網羅的**であるということです。すべての列挙型バリアントを処理しなければなりません。\
バリアントの処理を忘れると、Rustは**コンパイル時**にエラーを出して阻止します。

例えば、`ToDo`バリアントの処理を忘れた場合：

```rust
match self {
    Status::Done => true,
    Status::InProgress => false,
}
```


コンパイラが警告を出します：

```text
error[E0004]: non-exhaustive patterns: `ToDo` not covered
 --> src/main.rs:5:9
  |
5 |     match status {
  |     ^^^^^^^^^^^^ pattern `ToDo` not covered
```


これは非常に大きな利点です！\
コードベースは時間とともに進化します。例えば、後から`Blocked`のような新しいステータスを追加するかもしれません。Rustコンパイラは、
新しいバリアントに対するロジックが欠けているすべての`match`文に対してエラーを出力します。
だからこそ、Rust開発者はしばしば「コンパイラ駆動のリファクタリング」を絶賛するのです。コンパイラが次に何をすべきかを教えてくれ、
あとは報告された箇所を修正すればよいだけです。

<a id="catch-all"></a>

## キャッチオール

1つ以上のバリアントを気にしない場合は、`_`パターンをキャッチオールとして使用できます：

```rust
match status {
    Status::Done => true,
    _ => false
}
```


`_`パターンは、それまでのパターンでマッチしなかったあらゆるものにマッチします。

<div class="warning">
  `_`の利便性には代償が伴います。列挙型が変更された際、コンパイラが助けてくれることができなくなります。上記の例で`Blocked`バリアントが`Status`に追加された場合、この`match`は、新しい`Blocked`バリアントが`_`アームで暗黙的に処理されるため、意図した動作であるかどうかにかかわらず、そのままコンパイルされてしまいます。

正確性が重要な場合は、複数のアームが同じロジックを共有していても、
すべてのバリアントを明示的に記述してください（例：`Status::ToDo | Status::InProgress => false`）。そうすることで、新しいバリアントが追加されるたびに、意識的に判断を下すことになります。

</div>
