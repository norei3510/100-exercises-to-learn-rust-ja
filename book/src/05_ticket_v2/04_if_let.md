<a id="concise-branching"></a>

# 簡潔な分岐

前回の演習の解答は、おそらく次のようになっているでしょう：

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            Status::Done | Status::ToDo => {
                panic!(
                    "Only `In-Progress` tickets can be \
                    assigned to someone"
                )
            }
        }
    }
}
```


関心があるのは `Status::InProgress` バリアントだけです。
本当に他のすべてのバリアントをマッチさせる必要があるでしょうか？

そこで新しい構文の出番です！

<a id="if-let"></a>

## `if let`

`if let` 構文を使えば、他のすべてのバリアントを処理することなく、
列挙型の単一のバリアントだけをマッチさせることができます。

`if let` を使って `assigned_to` メソッドを簡略化する方法は以下の通りです：

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        if let Status::InProgress { assigned_to } = &self.status {
            assigned_to
        } else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        }
    }
}
```


<a id="letelse"></a>

## `let/else`

`else` 分岐で早期にリターンする場合（パニックも早期リターンに含まれます！）、
`let/else` 構文を使用できます：

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        let Status::InProgress { assigned_to } = &self.status else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        };
        assigned_to
    }
}
```


これにより、「ライトドリフト」を起こすことなく
構造分解した変数を代入できます。つまり、変数は直前のコードと同じインデントレベルで
代入されます。

<a id="style"></a>

## スタイル

`if let` と `let/else` はどちらも Rust の慣用的な構文です。\
コードの可読性を高めるために適宜使い分けてください。
ただし、やりすぎには注意しましょう。必要なときはいつでも `match` があります。
