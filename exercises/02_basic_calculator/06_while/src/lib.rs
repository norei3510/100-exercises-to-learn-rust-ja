// 階乗関数を`while`ループを用いて書き直してください。
pub fn factorial(n: u32) -> u32 {
    // `todo!()` マクロは、コンパイラが
    // 「後で処理する」と解釈するプレースホルダーであり、これにより
    // 型エラーが抑制されます。
    // 実行時にパニックが発生します。
    let mut result = 1;
    let mut i = 1;
    while i <= n {
        result *= i;
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
