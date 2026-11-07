// TODO: 新しい `SaturatingU16` 型を定義してください。
//   これは `u16` の値を保持する必要があります。
//   `u16`、`u8`、`&u16`、`&u8` からの型変換を提供する必要があります。
//   右辺の型が `SaturatingU16`、`u16`、`&u16`、`&SaturatingU16` である加算をサポートする必要があります。
//   加算は `u16` の最大値で飽和（saturate）する必要があります。
//   別の `SaturatingU16` または `u16` と比較できる必要があります。
//   デバッグ表現を出力できる必要があります。
//
// テストは `tests` フォルダ内にあります。型やメソッドの可視性に注意してください。

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        (*value).into()
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        (*value).into()
    }
}

impl std::ops::Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.value
    }
}

impl std::ops::Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        self + *rhs
    }
}

impl std::ops::Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        let sum = self.value.saturating_add(rhs);
        Self {
            value: sum,
        }
    }
}

impl std::ops::Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        self + *rhs
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
