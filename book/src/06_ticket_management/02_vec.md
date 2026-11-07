<a id="vectors"></a>

# ベクタ

配列の強みは同時に弱点でもあります。そのサイズは、コンパイル時にあらかじめ分かっていなければなりません。
実行時にしか分からないサイズで配列を作成しようとすると、コンパイルエラーが発生します：

```rust
let n = 10;
let numbers: [u32; n];
```


```text
error[E0435]: attempt to use a non-constant value in a constant
 --> src/main.rs:3:20
  |
2 | let n = 10;
3 | let numbers: [u32; n];
  |                    ^ non-constant value
```


チケット管理システムに配列は適していません。コンパイル時点で、いくつのチケットを保存する必要があるか分からないからです。
そこで登場するのが `Vec` です。

<a id="vec"></a>

## `Vec`

`Vec` は、標準ライブラリが提供する拡張可能な配列型です。\
`Vec::new` 関数を使用して、空の配列を作成できます：

```rust
let mut numbers: Vec<u32> = Vec::new();
```


その後、`push` メソッドを使用してベクタに要素をプッシュします：

```rust
numbers.push(1);
numbers.push(2);
numbers.push(3);
```


新しい値はベクタの末尾に追加されます。\
また、作成時に値が決まっている場合は、`vec!` マクロを使用して初期化済みのベクタを作成することもできます：

```rust
let numbers = vec![1, 2, 3];
```


<a id="accessing-elements"></a>

## 要素へのアクセス

要素へのアクセス構文は、配列の場合と同じです：

```rust
let numbers = vec![1, 2, 3];
let first = numbers[0];
let second = numbers[1];
let third = numbers[2];
```


インデックスは `usize` 型である必要があります。\
また、`get` メソッドを使用することもでき、これは `Option<&T>` を返します：

```rust
let numbers = vec![1, 2, 3];
assert_eq!(numbers.get(0), Some(&1));
// 範囲外のインデックスにアクセスしようとした場合、
// パニックではなく `None` が返されます。
assert_eq!(numbers.get(3), None);
```


アクセス時には、配列の要素アクセスと同様に境界チェックが行われます。計算量は O(1) です。

<a id="memory-layout"></a>

## メモリレイアウト

`Vec` はヒープ上に確保されるデータ構造です。\
`Vec` を作成すると、要素を格納するためにヒープ上にメモリが確保されます。

以下のコードを実行すると：

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
```


次のようなメモリレイアウトになります：

```text
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   2    |    3     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+
Heap:  | 1 | 2 | ? |
       +---+---+---+
```


`Vec` は、次の3つの情報を管理しています：

- 確保したヒープ領域への**ポインタ**。
- ベクタの**長さ**（つまり、ベクタに含まれる要素数）。
- ベクタの**容量**（つまり、ヒープ上に確保された領域に格納できる要素数）。

このレイアウトは見覚えがあるはずです。`String` と全く同じです！\
これは偶然ではありません。`String` は内部的に、バイトのベクタである `Vec<u8>` として定義されています：

```rust
pub struct String {
    vec: Vec<u8>,
}
```
