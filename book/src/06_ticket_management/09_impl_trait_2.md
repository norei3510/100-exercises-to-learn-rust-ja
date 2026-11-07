<a id="impl-trait-in-argument-position"></a>

# 引数位置における `impl Trait`

前のセクションでは、`impl Trait` を使用して型名を指定せずに型を返す方法を確認しました。\
同様の構文は**引数位置**でも使用できます：

```rust
fn print_iter(iter: impl Iterator<Item = i32>) {
    for i in iter {
        println!("{}", i);
    }
}
```


`print_iter` は `i32` のイテレータを受け取り、各要素を出力します。\
**引数位置**で使用される場合、`impl Trait` はトレイト境界を持つジェネリックパラメータと同等になります：

```rust
fn print_iter<T>(iter: T) 
where
    T: Iterator<Item = i32>
{
    for i in iter {
        println!("{}", i);
    }
}
```


<a id="downsides"></a>

## デメリット

原則として、引数位置では `impl Trait` よりもジェネリクスを優先してください。\
ジェネリクスでは、呼び出し側がターボフィッシュ構文 (`::<>`) を用いて引数の型を明示的に指定できるため、
曖昧さの解消に役立ちます。`impl Trait` ではそれができません。
