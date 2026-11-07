<a id="validation"></a>

# バリデーション

チケットの定義に戻ってみましょう：

```rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```


`Ticket` 構造体のフィールドには「raw」型を使用しています。
つまり、ユーザーはタイトルが空だったり、説明がめちゃくちゃ長かったり、あるいは
意味不明なステータス（例：「Funny」）のチケットを作成できてしまいます。\
もっと改善できます！

<a id="further-reading"></a>

## さらに詳しく

- [`String` のドキュメント](https://doc.rust-lang.org/std/string/struct.String.html)
  で、提供されているメソッドの詳細な概要を確認してください。演習で必要になります！
