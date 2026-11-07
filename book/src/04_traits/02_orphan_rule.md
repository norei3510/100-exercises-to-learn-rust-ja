<a id="implementing-traits"></a>

# トレイトの実装

型が別のクレート（例：Rust標準ライブラリの `u32`）で定義されている場合、
その型に対して直接新しいメソッドを定義することはできません。試そうとすると、

```rust
impl u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
```


コンパイラが次のようなエラーを出します：

```text
error[E0390]: cannot define inherent `impl` for primitive types
  |
1 | impl u32 {
  | ^^^^^^^^
  |
  = help: consider using an extension trait instead
```


<a id="extension-trait"></a>

## 拡張トレイト

**拡張トレイト**とは、`u32` のような外部の型に新しいメソッドを追加することを
主な目的とするトレイトのことです。
これは、前の演習で `IsEven` トレイトを定義し、
それを `i32` と `u32` に実装した際に用いたパターンそのものです。これにより、
`IsEven` がスコープ内にあれば、それらの型に対して `is_even` を自由に呼び出すことができます。

```rust
// トレイトをスコープ内に取り込みます
use my_library::IsEven;

fn main() {
    // それを実装している型でそのメソッドを呼び出します
    if 4.is_even() {
        // [...]
    }
}
```


<a id="one-implementation"></a>

## 単一の実装

記述できるトレイトの実装には制限があります。\
最も単純で明白な制限は、1つのクレート内で、同一の型に対して同じトレイトを2回実装することはできないということです。

例えば：

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        true
    }
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        false
    }
}
```


コンパイラはこれを拒否します：

```text
error[E0119]: conflicting implementations of trait `IsEven` for type `u32`
   |
5  | impl IsEven for u32 {
   | ------------------- first implementation here
...
11 | impl IsEven for u32 {
   | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u32`
```


`u32` の値に対して `IsEven::is_even` が呼び出された際、どのトレイト実装を使用すべきかについて曖昧さがあってはならず、
したがって実装は1つしか存在できません。

<a id="orphan-rule"></a>

## オーファンルール

複数のクレートが関わる場合、状況はより複雑になります。
具体的には、少なくとも以下のいずれかが満たされている必要があります：

- トレイトが現在のクレートで定義されている
- 実装対象の型が現在のクレートで定義されている

これはRustの**オーファンルール**として知られています。その目的は、メソッド解決プロセスを
一意にすることです。

次のような状況を想定してください：

- クレート `A` が `IsEven` トレイトを定義している
- クレート `B` が `u32` に対して `IsEven` を実装している
- クレート `C` が `u32` に対して（別の）`IsEven` トレイトの実装を提供している
- クレート `D` が `B` と `C` の両方に依存し、`1.is_even()` を呼び出す

どの実装を使用すべきでしょうか？ `B` で定義されたものでしょうか？ それとも `C` で定義されたものでしょうか？\
明確な答えはありません。そのため、このようなシナリオを防ぐためにオーファンルールが定義されました。
オーファンルールのおかげで、クレート `B` もクレート `C` もコンパイルできません。

<a id="further-reading"></a>

## さらに詳しく

- 上述のオーファンルールには、いくつかの注意点や例外があります。
  その詳細について知りたい場合は、[リファレンス](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence)
  を確認してください。
