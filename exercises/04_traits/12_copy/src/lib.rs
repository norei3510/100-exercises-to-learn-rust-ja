// TODO: テストがコンパイルされ通過するように、必要なトレイトを実装してください。
//  テスト自体は変更*できません*。

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::ops::Add for WrappingU32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value.wrapping_add(rhs.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
