<a id="errorsource"></a>

# `Error::source`

`Error` トレイトの解説を完結させるために、もう一つ触れておくべき点があります。それが `source` メソッドです。

```rust
// 今回は完全な定義です！
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```


`source` メソッドは、**エラーの原因**（もしあれば）にアクセスするための手段です。\
エラーはしばしば連鎖します。つまり、あるエラーが別のエラーの原因となるということです。例えば、上位レベルのエラー（例：
データベースに接続できない）が、下位レベルのエラー（例：データベースのホスト名を解決できない）によって引き起こされるといった具合です。
`source` メソッドを使えば、エラーの連鎖全体を「辿る」ことができ、ログにエラーコンテキストを記録する際によく利用されます。

<a id="implementing-source"></a>

## `source` の実装

`Error` トレイトは、常に `None`（つまり根本的な原因がないこと）を返すデフォルト実装を提供しています。そのため、
前回の演習では `source` を意識する必要がありませんでした。\
このデフォルト実装をオーバーライドすることで、エラー型に原因を持たせることができます。

```rust
use std::error::Error;

#[derive(Debug)]
struct DatabaseError {
    source: std::io::Error
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to connect to the database")
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```


この例では、`DatabaseError` が `std::io::Error` をソースとしてラップしています。
そして、`source` メソッドをオーバーライドし、呼び出された際にこのソースを返すようにしています。

<a id="dyn-error--static"></a>

## `&(dyn Error + 'static)`

この `&(dyn Error + 'static)` 型とは何でしょうか？\
詳しく見ていきましょう：

- `dyn Error` は **トレイトオブジェクト** です。これは、`Error` トレイトを実装している任意の型を参照するための方法です。
- `'static` は特殊な **ライフタイム指定子** です。
  `'static` は、その参照が「必要な限り」、つまりプログラムの実行全体を通じて有効であることを意味します。

これらを組み合わせると、`&(dyn Error + 'static)` は `Error` トレイトを実装し、
プログラムの実行全体を通じて有効なトレイトオブジェクトへの参照となります。

今のところ、これらの概念についてあまり深く気にする必要はありません。今後の章でより詳しく解説します。

<a id="implementing-source-using-thiserror"></a>

## `thiserror` を使った `source` の実装

`thiserror` では、エラー型に対して `source` を自動的に実装するための3つの方法が提供されています：

- `source` という名前のフィールドは、自動的にエラーのソースとして使用されます。
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          source: std::io::Error
      }
  }
  ```
- `#[source]` 属性が付いたフィールドは、自動的にエラーのソースとして使用されます。
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[source]
          inner: std::io::Error
      }
  }
  ```
- `#[from]` 属性が付いたフィールドは、自動的にエラーのソースとして使用され、**さらに**
  `thiserror` が、アノテーションされた型を自身のエラー型に変換するための `From` 実装を自動的に生成します。
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[from]
          inner: std::io::Error
      }
  }
  ```

<a id="the--operator"></a>

## `?` 演算子

`?` 演算子は、エラーを伝播させるための短縮記法です。\
`Result` を返す関数で使用した場合、`Result` が `Err` であれば、エラーを返して早期に処理を終了します。

例えば：

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```


は、以下と等価です：

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = match File::open("file.txt") {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    }
    Ok(contents)
}
```


`?` 演算子を使えば、エラー処理のコードを大幅に短縮できます。\
特に `?` 演算子は、変換が可能であれば（つまり適切な `From` 実装があれば）、失敗しうる操作のエラー型を
関数のエラー型に自動的に変換します。
