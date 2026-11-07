<a id="derive-macros"></a>

# Derive マクロ

`Ticket` に `PartialEq` を実装するのは、少し面倒でしたね。
構造体の各フィールドを手動で比較する必要がありました。

<a id="destructuring-syntax"></a>

## デストラクチャリング構文

さらに、この実装は壊れやすく、構造体の定義が変更された場合
（例：新しいフィールドが追加された場合）、`PartialEq` の実装を更新することを忘れてはいけません。

構造体をフィールドに**デストラクチャリング**することで、そのリスクを軽減できます：

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        // [...]
    }
}
```


`Ticket` の定義が変更されると、デストラクチャリングが網羅的ではなくなったとして、
コンパイラがエラーを出します。\
また、変数のシャドウイングを避けるために、構造体フィールドの名前を変更することもできます：

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        let Ticket {
            title: other_title,
            description: other_description,
            status: other_status,
        } = other;
        // [...]
    }
}
```


デストラクチャリングはツールキットに入れておくと便利なパターンですが、
さらに便利な方法があります。それが**derive マクロ**です。

<a id="macros"></a>

## マクロ

これまでの演習で、すでにいくつかのマクロに触れてきました：

- テストケース内の `assert_eq!` と `assert!`
- コンソールに出力するための `println!`

Rust のマクロは**コードジェネレータ**です。\
提供した入力に基づいて新しい Rust コードを生成し、その生成されたコードがプログラムの他の部分と共に
コンパイルされます。一部のマクロは Rust の標準ライブラリに組み込まれていますが、
自分で作成することも可能です。このコースでは独自のマクロは作成しませんが、
["Further reading" セクション](#further-reading) に有用なヒントが記載されています。

<a id="inspection"></a>

### 確認

一部の IDE では、マクロを展開して生成されたコードを確認できます。それができない場合は、
[`cargo-expand`](https://github.com/dtolnay/cargo-expand) を使用できます。

<a id="derive-macros-1"></a>

### Derive マクロ

**derive マクロ**は、Rust マクロの一種です。これは構造体の上に記述される**属性**として指定されます。

```rust
#[derive(PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String
}
```


derive マクロは、カスタム型に対して一般的（かつ「自明な」）トレイトの実装を自動化するために使用されます。
上記の例では、`PartialEq` トレイトが `Ticket` に対して自動的に実装されています。
マクロを展開すると、生成されたコードは手動で書いたものと機能的に同等であることがわかります。
ただし、読むには少し煩雑です：

```rust
#[automatically_derived]
impl ::core::cmp::PartialEq for Ticket {
    #[inline]
    fn eq(&self, other: &Ticket) -> bool {
        self.title == other.title 
            && self.description == other.description
            && self.status == other.status
    }
}
```


コンパイラは、可能な場合にはトレイトを derive するように促してくれます。

<a id="further-reading"></a>

## Further reading

- [The little book of Rust macros](https://veykril.github.io/tlborm/)
- [Proc macro workshop](https://github.com/dtolnay/proc-macro-workshop)
