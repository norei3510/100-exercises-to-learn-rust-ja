<a id="unwrapping"></a>

# Unwrapping

`Ticket::new` は、無効な入力に対してパニックを起こす代わりに、`Result` を返すようになりました。\
これは呼び出し側にとってどのような意味を持つのでしょうか？

<a id="failures-cant-be-implicitly-ignored"></a>

## 失敗を（暗黙的に）無視できない

例外とは異なり、Rust の `Result` では **呼び出し側でエラーを処理する** ことが強制されます。\
`Result` を返す関数を呼び出した場合、Rust はエラーケースを暗黙的に無視することを許可しません。

```rust
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}

// これはコンパイルできません。エラーケースを処理していないためです。
// `match` または `Result` が提供するコンビネータのいずれかを使用して、
// 成功値を「アンラップ」するか、エラーを処理する必要があります。
let number = parse_int("42") + 2;
```


<a id="you-got-a-result-now-what"></a>

## `Result` が返されました。次はどうすればいいでしょうか？

`Result` を返す関数を呼び出す際、主に 2 つの選択肢があります。

- 操作が失敗した場合にパニックを起こす。
  これは `unwrap` または `expect` メソッドを使用して行います。
  ```rust
  // `parse_int` が `Err` を返した場合にパニックが発生します。
  let number = parse_int("42").unwrap();
  // `expect` を使用すると、カスタムのパニックメッセージを指定できます。
  let number = parse_int("42").expect("Failed to parse integer");
  ```
- `match` 式を用いて `Result` を分解し、エラーケースを明示的に処理する。
  ```rust
  match parse_int("42") {
      Ok(number) => println!("Parsed number: {}", number),
      Err(err) => eprintln!("Error: {}", err),
  }
  ```
