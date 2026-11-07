<a id="indexing"></a>

# インデックス指定

`TicketStore::get` は、指定された `TicketId` に対して `Option<&Ticket>` を返します。\
これまで、Rust のインデックス指定構文を用いて配列やベクタの要素にアクセスする方法を見てきました：

```rust
let v = vec![0, 1, 2];
assert_eq!(v[0], 0);
```


`TicketStore` でも同様の操作を実現するにはどうすればよいでしょうか？\
ご想像の通り、トレイト `Index` を実装する必要があります！

<a id="index"></a>

## `Index`

`Index` トレイトは Rust の標準ライブラリで定義されています：

```rust
// 若干簡略化されています
pub trait Index<Idx>
{
    type Output;

    // 必須メソッド
    fn index(&self, index: Idx) -> &Self::Output;
}
```


このトレイトには以下があります：

- インデックスの型を表すジェネリックパラメータ `Idx`
- インデックスを用いて取得した型を表す関連型 `Output`

`index` メソッドが `Option` を返さない点に注目してください。配列やベクタのインデックス指定と同様に、存在しない要素にアクセスしようとした場合には `index` がパニックを起こすことが想定されています。
