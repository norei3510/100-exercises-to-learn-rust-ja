<a id="variants-can-hold-data"></a>

# バリアントはデータを保持できる

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```


私たちの `Status` 列挙型は、一般的に **C言語スタイルの列挙型** と呼ばれるものです。\
各バリアントは単純なラベルであり、名前付き定数のようなものです。この種の列挙型は、C、C++、Java、C#、Python など、多くのプログラミング
言語で見られます。

しかし、Rust の列挙型はさらに踏み込んだことができます。**各バリアントにデータを紐付ける** ことができるのです。

<a id="variants"></a>

## バリアント

例えば、現在チケットに取り組んでいる人物の名前を保存したいとしましょう。\
この情報は、チケットが進行中の場合にのみ存在します。ToDo チケットや
完了済みチケットには存在しません。
これは、`InProgress` バリアントに `String` フィールドを紐付けることでモデル化できます：

```rust
enum Status {
    ToDo,
    InProgress {
        assigned_to: String,
    },
    Done,
}
```


`InProgress` は今や **構造体のようなバリアント** です。\
実際、その構文は構造体を定義する際に使用したものと同じであり、単に列挙型の中にバリアントとして「インライン化」されているだけです。

<a id="accessing-variant-data"></a>

## バリアントデータへのアクセス

`Status` インスタンスの `assigned_to` にアクセスしようとすると、

```rust
let status: Status = /* */;

// これはコンパイルできません。
println!("Assigned to: {}", status.assigned_to);
```


コンパイラがこれを阻止します：

```text
error[E0609]: no field `assigned_to` on type `Status`
 --> src/main.rs:5:40
  |
5 |     println!("Assigned to: {}", status.assigned_to);
  |                                        ^^^^^^^^^^^ unknown field
```


`assigned_to` は **バリアント固有** のものであり、すべての `Status` インスタンスで利用できるわけではありません。\
`assigned_to` にアクセスするには、**パターンマッチング** を使用する必要があります：

```rust
match status {
    Status::InProgress { assigned_to } => {
        println!("Assigned to: {}", assigned_to);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```


<a id="bindings"></a>

## バインディング

マッチパターン `Status::InProgress { assigned_to }` において、`assigned_to` は **バインディング** です。\
`Status::InProgress` バリアントを **分解** し、`assigned_to` フィールドを
`assigned_to` という名前の新しい変数にバインドしています。\
必要に応じて、フィールドを別の変数名にバインドすることも可能です：

```rust
match status {
    Status::InProgress { assigned_to: person } => {
        println!("Assigned to: {}", person);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```
