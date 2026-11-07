// デストラクタに関する適切な演習問題を作成するには、さらにいくつかの仕組みが必要です。
// この概念については、トレイトと内部可変性を扱った後の章で改めて取り上げます。
fn outro() -> &'static str {
    "I have a basic understanding of __!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
