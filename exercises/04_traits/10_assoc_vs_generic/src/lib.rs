// TODO: `self` を `n` 乗するメソッド `power` を持つ新しいトレイト `Power` を定義してください。
//  トレイトの定義とその実装があれば、テストをコンパイルしてパスさせるのに十分であるはずです。
//
// 推奨事項：すべてのケースを一括して処理する汎用的な実装を書きたくなるかもしれません。しかし、これはかなり複雑であり、追加のクレート（すなわち `num-traits`）を使用する必要があります。
// それでも、高度に汎用的な実装に伴う複雑さを避けるため、代わりにシンプルなマクロを使用する方が好ましい場合があります。詳細について知りたい場合は、『Little book of Rust macros』(https://veykril.github.io/tlborm/) を参照してください。
// とはいえ、必ずしもそうする必要はありません。3つの個別の実装を手動で記述しても全く問題ありません。興味がある場合にのみ、さらに踏み込んでみてください。

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
