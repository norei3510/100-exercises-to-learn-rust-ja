<a id="traits"></a>

# トレイト

改めて `Ticket` 型を見てみましょう：

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}
```


これまでのテストはすべて、`Ticket` のフィールドを用いてアサーションを行ってきました。

```rust
assert_eq!(ticket.title(), "A new title");
```


もし、2つの `Ticket` インスタンスを直接比較したい場合はどうなるでしょうか？

```rust
let ticket1 = Ticket::new(/* ... */);
let ticket2 = Ticket::new(/* ... */);
ticket1 == ticket2
```


コンパイラがこれを阻止します：

```text
error[E0369]: binary operation `==` cannot be applied to type `Ticket`
  --> src/main.rs:18:13
   |
18 |     ticket1 == ticket2
   |     ------- ^^ ------- Ticket
   |     |
   |     Ticket
   |
note: an implementation of `PartialEq` might be missing for `Ticket`
```


`Ticket` は新しい型です。デフォルトの状態では、**いかなる振る舞いも定義されていません**。\
Rustは、`Ticket` インスタンスに `String` が含まれているという理由だけで、2つのインスタンスをどう比較すべきかを魔法のように推論することはありません。

とはいえ、Rustコンパイラは正しい方向へと導いてくれています。`PartialEq` の実装が
不足している可能性があることを示唆しているのです。`PartialEq` は**トレイト**です！

<a id="what-are-traits"></a>

## トレイトとは？

トレイトは、Rustにおける**インターフェース**を定義するための仕組みです。\
トレイトは、その契約を満たすために型が実装しなければならないメソッドの集合を定義します。

<a id="defining-a-trait"></a>

### トレイトの定義

トレイトの定義構文は以下の通りです：

```rust
trait <TraitName> {
    fn <method_name>(<parameters>) -> <return_type>;
}
```


例えば、実装者に `is_zero` メソッドの定義を要求する `MaybeZero` という名前のトレイトを定義できます：

```rust
trait MaybeZero {
    fn is_zero(self) -> bool;
}
```


<a id="implementing-a-trait"></a>

### トレイトの実装

ある型にトレイトを実装するには、通常の [^inherent] メソッドと同様に `impl` キーワードを使用しますが、
構文が少し異なります：

```rust
impl <TraitName> for <TypeName> {
    fn <method_name>(<parameters>) -> <return_type> {
        // メソッド本体
    }
}
```


例えば、カスタム数値型 `WrappingU32` に `MaybeZero` トレイトを実装する場合：

```rust
pub struct WrappingU32 {
    inner: u32,
}

impl MaybeZero for WrappingU32 {
    fn is_zero(self) -> bool {
        self.inner == 0
    }
}
```


<a id="invoking-a-trait-method"></a>

### トレイトメソッドの呼び出し

トレイトメソッドを呼び出すには、通常のメソッドと同様に `.` 演算子を使用します：

```rust
let x = WrappingU32 { inner: 5 };
assert!(!x.is_zero());
```


トレイトメソッドを呼び出すには、次の2つの条件が満たされている必要があります：

- その型がトレイトを実装していること。
- トレイトがスコープ内にあること。

後者の条件を満たすために、トレイトに対する `use` ステートメントを追加する必要がある場合があります：

```rust
use crate::MaybeZero;
```


以下の場合は、この記述は不要です：

- トレイトが、呼び出しが行われているのと同じモジュール内で定義されている場合。
- トレイトが標準ライブラリの**prelude**で定義されている場合。
  preludeとは、すべてのRustプログラムに自動的にインポートされるトレイトと型の集合です。
  いわば、すべてのRustモジュールの冒頭に `use std::prelude::*;` が追加されたようなものです。

preludeに含まれるトレイトと型の一覧は、
[Rustドキュメント](https://doc.rust-lang.org/std/prelude/index.html) で確認できます。

[^inherent]: トレイトを使用せず、型に直接定義されたメソッドは、**固有メソッド**とも呼ばれます。
