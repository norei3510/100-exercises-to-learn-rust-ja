<a id="hashmap"></a>

# `HashMap`

`Index`/`IndexMut` の実装は最適とは言えません。IDでチケットを取得するには `Vec` 全体を走査する必要があり、アルゴリズムの計算量は `O(n)` となります。ここで `n` はストア内のチケット数です。

チケットの格納に別のデータ構造、すなわち `HashMap<K, V>` を使用することで、効率を向上させることができます。

```rust
use std::collections::HashMap;

// 型推論により、明示的な型シグネチャ（この例では
// `HashMap<String, String>`）を省略できます。
let mut book_reviews = HashMap::new();

book_reviews.insert(
    "Adventures of Huckleberry Finn".to_string(),
    "My favorite book.".to_string(),
);
```


`HashMap` はキーと値のペアを扱います。これは両方に対してジェネリックであり、`K` がキー型のジェネリックパラメータ、`V` が値型のジェネリックパラメータとなります。

挿入、取得、削除の想定コストは **定数** であり、`O(1)` です。
これは私たちのユースケースにとって理想的に思えませんか？

<a id="key-requirements"></a>

## 主要な要件

`HashMap` の構造体定義にはトレイト境界はありませんが、メソッドにはいくつか設定されています。例として `insert` を見てみましょう。

```rust
// 若干簡略化されています
impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        // [...]
    }
}
```


キー型は `Eq` および `Hash` トレイトを実装している必要があります。\
これら2つのトレイトについて詳しく見ていきましょう。

<a id="hash"></a>

## `Hash`

ハッシュ関数（またはハッシャー）は、潜在的に無限の値の集合（例：考えられるすべての文字列）を、限定された範囲（例：`u64` の値）にマッピングします。\
世の中には多くの異なるハッシュ関数が存在し、それぞれ異なる特性（速度、衝突リスク、可逆性など）を持っています。

`HashMap` は、その名の通り、内部でハッシュ関数を使用します。
キーをハッシュ化し、そのハッシュ値を用いて対応する値を格納・取得します。
この戦略ではキー型がハッシュ可能である必要があるため、`K` に `Hash` トレイト境界が設けられています。

`Hash` トレイトは `std::hash` モジュールにあります。

```rust
pub trait Hash {
    // 必須メソッド
    fn hash<H>(&self, state: &mut H)
       where H: Hasher;
}
```


`Hash` を手動で実装することは滅多にありません。ほとんどの場合は derive します。

```rust
#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
}
```


<a id="eq"></a>

## `Eq`

`HashMap` はキーの等価性を比較できなければなりません。これは特にハッシュ衝突（つまり、2つの異なるキーが同じハッシュ値になること）に対処する際に重要です。

「それは `PartialEq` トレイトの役割ではないか」と思うかもしれません。ほぼ正解です！\
`PartialEq` だけでは `HashMap` には不十分です。なぜなら、反射律（すなわち `a == a` は常に `true` であること）が保証されていないからです。\
例えば、浮動小数点数（`f32` および `f64`）は `PartialEq` を実装していますが、反射律の特性は満たしていません。`f32::NAN == f32::NAN` は `false` となります。\
反射律は `HashMap` が正しく動作するために不可欠です。これがなければ、挿入時に使用したのと同じキーを使ってマップから値を取得することができません。

`Eq` トレイトは、`PartialEq` に反射律の特性を追加したものです。

```rust
pub trait Eq: PartialEq {
    // 追加のメソッドはありません
}
```


これはマーカートレイトです。新しいメソッドを追加するものではなく、`PartialEq` で実装された等価性ロジックが反射的であることをコンパイラに伝えるための手段です。

`PartialEq` を derive する際に、`Eq` を自動的に derive できます。

```rust
#[derive(PartialEq, Eq)]
struct Person {
    id: u32,
    name: String,
}
```


<a id="eq-and-hash-are-linked"></a>

## `Eq` と `Hash` は連動している

`Eq` と `Hash` の間には暗黙の契約があります。すなわち、2つのキーが等しい場合、それらのハッシュ値も等しくなければなりません。
これは `HashMap` が正しく動作するために不可欠です。この契約を破ると、`HashMap` を使用した際に意味をなさない結果が得られます。
