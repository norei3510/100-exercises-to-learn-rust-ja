<a id="deref-trait"></a>

# `Deref` トレイト

前回の演習では、特にやることはありませんでしたよね？

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }
}
```


を

```rust
impl Ticket {
    pub fn title(&self) -> &str {
        &self.title
    }
}
```


に変更するだけで、コードはコンパイルされ、テストもパスしました。
しかし、ここで少し違和感を覚えるはずです。

<a id="it-shouldnt-work-but-it-does"></a>

## 本来動くはずはないが、実際には動く

事実を整理してみましょう：

- `self.title` は `String` です
- したがって、`&self.title` は `&String` となります
- （修正後の）`title` メソッドの出力は `&str` です

コンパイラエラーが出ると予想しますよね？ `Expected &String, found &str` かそれに類するエラーが。
ところが、実際にはそのまま動作してしまいます。**なぜ**でしょうか？

<a id="deref-to-the-rescue"></a>

## `Deref` が解決してくれる

`Deref` トレイトは、[**deref coercion**](https://doc.rust-lang.org/std/ops/trait.Deref.html#deref-coercion) として知られる言語機能の仕組みです。\
このトレイトは標準ライブラリの `std::ops` モジュールで定義されています：

```rust
// とりあえず定義を少し簡略化しました。
// 完全な定義は後ほど見ていきます。
pub trait Deref {
    type Target;
    
    fn deref(&self) -> &Self::Target;
}
```


`type Target` は **associated type** です。\
これは、トレイトを実装する際に指定しなければならない具体的な型のプレースホルダーです。

<a id="deref-coercion"></a>

## Deref coercion

型 `T` に対して `Deref<Target = U>` を実装することで、`&T` と `&U` が
ある程度相互に置き換え可能であることをコンパイラに伝えることになります。\
具体的には、以下のような挙動になります：

- `T` への参照は、暗黙的に `U` への参照に変換されます（つまり `&T` が `&U` になります）
- `&T` に対して、`&self` を入力として受け取る `U` で定義されたすべてのメソッドを呼び出すことができます。

デリファレンス演算子 `*` に関してもう一点ありますが、今のところは必要ありません（気になる方は `std` のドキュメントを
参照してください）。

<a id="string-implements-deref"></a>

## `String` が `Deref` を実装する

`String` は `Target = str` を用いて `Deref` を実装しています：

```rust
impl Deref for String {
    type Target = str;
    
    fn deref(&self) -> &str {
        // [...]
    }
}
```


この実装と deref coercion のおかげで、必要に応じて `&String` は自動的に `&str` に変換されます。

<a id="dont-abuse-deref-coercion"></a>

## Deref coercion を乱用しない

Deref coercion は強力な機能ですが、混乱を招く可能性があります。\
型を自動的に変換すると、コードの可読性や理解しやすさが損なわれることがあります。もし `T` と `U` の両方に同じ名前の
メソッドが定義されていた場合、どちらが呼び出されるのでしょうか？

本コースの後半で、deref coercion の「最も安全な」ユースケースであるスマートポインタについて検討します。
