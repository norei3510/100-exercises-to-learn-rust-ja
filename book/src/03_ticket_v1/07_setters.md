<a id="mutable-references"></a>

# 可変参照

アクセサメソッドは、現在次のようになっているはずです。

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}
```


ところどころに `&` を散りばめたところ、うまくいきました！\
これで、`Ticket` のインスタンスを消費することなくフィールドにアクセスする方法が整いました。
次に、**セッターメソッド**を使って `Ticket` 構造体をどのように拡張できるか見ていきましょう。

<a id="setters"></a>

## セッター

セッターメソッドを使用すると、不変条件が守られていることを保証しつつ、`Ticket` のプライベートフィールドの値を変更できます（つまり、`Ticket` のタイトルを空文字列に設定することはできません）。

Rustでセッターを実装する一般的な方法は2つあります。

- `self` を入力として受け取る。
- `&mut self` を入力として受け取る。

<a id="taking-self-as-input"></a>

### `self` を入力として受け取る

1つ目のアプローチは次のようになります。

```rust
impl Ticket {
    pub fn set_title(mut self, new_title: String) -> Self {
        // 新しいタイトルを検証してください [...]
        self.title = new_title;
        self
    }
}
```


これは `self` の所有権を取得し、タイトルを変更して、修正後の `Ticket` インスタンスを返します。\
使い方は次の通りです。

```rust
let ticket = Ticket::new(
    "Title".into(), 
    "Description".into(), 
    "To-Do".into()
);
let ticket = ticket.set_title("New title".into());
```


`set_title` は `self` の所有権を取得する（つまり **消費する**）ため、結果を変数に再代入する必要があります。
上記の例では、**変数のシャドウイング**を利用して同じ変数名を再利用しています。既存の変数と同じ名前で新しい変数を宣言すると、新しい変数が古い変数を**シャドウ**します。これはRustコードでよく見られるパターンです。

`self`-セッターは、複数のフィールドを一度に変更したい場合に非常に便利です。複数の呼び出しをチェーンさせることができるからです！

```rust
let ticket = ticket
    .set_title("New title".into())
    .set_description("New description".into())
    .set_status("In Progress".into());
```


<a id="taking-mut-self-as-input"></a>

### `&mut self` を入力として受け取る

`&mut self` を使用する2つ目のセッターアプローチは、代わりに次のようになります。

```rust
impl Ticket {
    pub fn set_title(&mut self, new_title: String) {
        // 新しいタイトルを検証してください [...]
        
        self.title = new_title;
    }
}
```


この場合、メソッドは `self` への可変参照を入力として受け取り、タイトルを変更して終了します。
何も返されません。

使い方は次の通りです。

```rust
let mut ticket = Ticket::new(
    "Title".into(),
    "Description".into(),
    "To-Do".into()
);
ticket.set_title("New title".into());

// 修正済みのチケットを使用してください
```


所有権は呼び出し元に留まるため、元の `ticket` 変数は引き続き有効です。結果を再代入する必要はありません。
ただし、可変参照を取得するため、`ticket` を可変として宣言する必要があります。

`&mut`-セッターには欠点があります。複数の呼び出しをチェーンさせることができません。
修正後の `Ticket` インスタンスを返さないため、1つ目のセッターの結果に対して別のセッターを呼び出すことはできません。
各セッターを個別に呼び出す必要があります。

```rust
ticket.set_title("New title".into());
ticket.set_description("New description".into());
ticket.set_status("In Progress".into());
```
