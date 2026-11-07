<a id="ordering"></a>

# 順序付け

`Vec`から`HashMap`に移行したことで、チケット管理システムのパフォーマンスが向上し、
同時にコードも簡素化されました。\
しかし、すべてが順調というわけではありません。`Vec`ベースのストアを反復処理する場合、チケットは
追加された順序で返されることが保証されていました。\
`HashMap`ではそうはいきません。チケットを反復処理することはできますが、その順序はランダムになります。

`HashMap`から`BTreeMap`に切り替えることで、一貫した順序を復元できます。

<a id="btreemap"></a>

## `BTreeMap`

`BTreeMap`は、エントリがキーによってソートされることを保証します。\
これは、エントリを特定の順序で反復処理する必要がある場合や、
範囲クエリ（例：「IDが10から20までのすべてのチケットを取得」）を実行する必要がある場合に有用です。

`HashMap`と同様に、`BTreeMap`の定義にトレイト境界はありません。
しかし、そのメソッドにはトレイト境界が指定されています。`insert`を見てみましょう：

```rust
// `K` と `V` は、それぞれキー型と値型を表しており、
// `HashMap` と同様です。
impl<K, V> BTreeMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        // 実装
    }
}
```


`Hash`はもう必要ありません。代わりに、キー型が`Ord`トレイトを実装している必要があります。

<a id="ord"></a>

## `Ord`

`Ord`トレイトは、値の比較に使用されます。\
`PartialEq`が等価性の比較に使用されるのに対し、`Ord`は順序の比較に使用されます。

これは`std::cmp`で定義されています：

```rust
pub trait Ord: Eq + PartialOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}
```


`cmp`メソッドは`Ordering`列挙型を返し、これは
`Less`、`Equal`、または`Greater`のいずれかになります。\
`Ord`では、他の2つのトレイト、`Eq`と`PartialOrd`の実装が必要です。

<a id="partialord"></a>

## `PartialOrd`

`PartialOrd`は`Ord`の弱いバージョンであり、`PartialEq`が`Eq`の弱いバージョンであるのと同様です。
その理由は、定義を見ればわかります：

```rust
pub trait PartialOrd: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}
```


`PartialOrd::partial_cmp`は`Option`を返しますが、2つの値が比較可能であるとは限りません。\
例えば、`f32`は`Ord`を実装していません。なぜなら`NaN`の値は比較不可能であり、
これは`f32`が`Eq`を実装していない理由と同じです。

<a id="implementing-ord-and-partialord"></a>

## `Ord`と`PartialOrd`の実装

`Ord`と`PartialOrd`はどちらも、型に対して派生させることができます。

```rust
// `Ord`で必要となるため、
// `Eq`と`PartialEq`も追加する必要があります。
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct TicketId(u64);
```


手動で実装することを選択した（あるいは必要となった）場合は、以下の点に注意してください：

- `Ord`と`PartialOrd`は、`Eq`および`PartialEq`と整合している必要があります。
- `Ord`と`PartialOrd`は、互いに整合している必要があります。
