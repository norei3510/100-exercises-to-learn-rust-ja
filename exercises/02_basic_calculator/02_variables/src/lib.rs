// 👇 `///` から始まる以下の行は、**ドキュメントコメント**と呼ばれます。
//    これらは、後続のアイテムにドキュメントを付与します。この場合は `speed` 関数です。
//    この演習のディレクトリから `cargo doc --open` を実行すると、Rust が
//    これらのコメントから HTML ドキュメントを生成し、ブラウザで開きます。

/// 旅の出発点と終点、および完了までにかかった時間が与えられたとき、
/// 平均速度を計算してください。
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: テストをパスさせるために、適切な値を持つ `distance` という名前の変数を定義してください。
    //  `distance` に型アノテーションを付ける必要はありますか？その理由も併せて教えてください。
    let distance = end - start;

    // 以下の行は変更しないでください
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}
