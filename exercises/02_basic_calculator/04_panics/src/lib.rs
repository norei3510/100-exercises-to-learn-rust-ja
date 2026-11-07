/// 旅の出発点と終点、および旅に要した時間が与えられたとき、
/// 旅の平均速度を計算してください。
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: `time_elapsed` が 0 の場合はカスタムメッセージでパニックする

    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 `#[should_panic]` アノテーションを使用すると、テスト対象のコードが
    //    パニックを起こすことを想定していると明示できます。また、`expected` を使ってパニックメッセージを確認することも可能です。
    //    これらはすべて、Rust標準のテストフレームワークの一部です！
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
