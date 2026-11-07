fn compute(a: u32, b: u32) -> u32 {
    let multiplier: u32 = 4;
    // ^ `: u32` を完全に省略することも可能です。
    // コンパイラは、以下の用法に基づいて正しい型を推論します。
    a + b * multiplier
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
