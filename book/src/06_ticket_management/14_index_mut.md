<a id="mutable-indexing"></a>

# 可変インデックス指定

`Index` は読み取り専用アクセスを許可します。取得した値を
変更することはできません。

<a id="indexmut"></a>

## `IndexMut`

可変性を許可したい場合は、`IndexMut` トレイトを実装する必要があります。

```rust
// 若干簡略化されています
pub trait IndexMut<Idx>: Index<Idx>
{
    // 必須メソッド
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```


`IndexMut` は、_追加の_機能を有効にするため、型がすでに `Index` を実装している場合にのみ実装可能です。
