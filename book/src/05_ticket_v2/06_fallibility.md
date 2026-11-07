<a id="fallibility"></a>

# 失敗可能性

前回の演習で扱った `Ticket::new` 関数を振り返ってみましょう：

```rust
impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: Status
    ) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Ticket {
            title,
            description,
            status,
        }
    }
}
```


いずれかのチェックが失敗した時点で、関数はパニックします。
これは、呼び出し側に**エラーを処理**する機会を与えないため、理想的ではありません。

ここで、Rustの主要なエラー処理メカニズムである `Result` 型を導入しましょう。

<a id="the-result-type"></a>

## `Result` 型

`Result` 型は、標準ライブラリで定義されている列挙型です：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```


この型には2つのバリアントがあります：

- `Ok(T)`: 操作の成功を表します。操作の結果である `T` を保持します。
- `Err(E)`: 操作の失敗を表します。発生したエラーである `E` を保持します。

`Ok` と `Err` はどちらもジェネリックであり、成功時とエラー時にそれぞれ独自の型を指定できます。

<a id="no-exceptions"></a>

## 例外はない

Rustにおける回復可能なエラーは、**値として表現**されます。\
これらは単なる型のインスタンスであり、他の値と同様に受け渡しや操作が行われます。
これは、**例外**を用いてエラーを通知するPythonやC#などの他の言語との大きな違いです。

例外は独立した制御フロー経路を作り出すため、その挙動を推論するのが難しくなることがあります。\
関数のシグネチャを見ただけでは、その関数が例外をスローする可能性があるかどうかが分かりません。
また、関数のシグネチャを見ただけでは、**どの**例外型をスローし得るのかも分かりません。\
それを知るには、関数のドキュメントを読むか、実装を確認する必要があります。

例外処理のロジックは局所性が非常に低いです。例外をスローするコードと、それをキャッチするコードが
遠く離れており、両者の間に直接的な結びつきがありません。

<a id="fallibility-is-encoded-in-the-type-system"></a>

## 失敗可能性は型システムにエンコードされる

Rustは `Result` により、**失敗可能性を関数のシグネチャにエンコード**することを強制します。\
関数が失敗する可能性がある場合（そして呼び出し側にエラー処理の機会を与えたい場合）、その関数は `Result` を返す必要があります。

```rust
// シグネチャを見るだけで、この関数が
// 失敗する可能性があることがわかります。また、`ParseIntError`を確認することで、
// どのような失敗が想定されるかを把握できます。
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}
```


これが `Result` の大きな利点です。失敗可能性を明示的にできるのです。

ただし、パニックも存在することに注意してください。パニックは、他の言語の例外と同様に型システムでは追跡されません。
しかし、パニックは**回復不能なエラー**を想定したものであり、控えめに使用すべきです。
