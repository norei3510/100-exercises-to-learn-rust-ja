// TODO: いわゆる「Drop bomb」を実装してください。これは、特定の操作が行われていない限り、
// ドロップ時にパニックを起こす型です。
// 想定されるAPIは、以下のテストで確認できます。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // 爆弾は投下時にパニックを起こすべきです。
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // 爆弾は解除済みであるため、投下時にパニックを起こしてはなりません。
    }
}
