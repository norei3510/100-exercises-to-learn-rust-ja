#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // 最大容量に達しました
        assert_eq!(v.capacity(), 2);

        v.push(3); // 容量超過のため、リサイズが必要です

        // 新しい容量がどうなるか予想できますか？
        // 標準ライブラリはベクトルのサイズ変更に使用されるアルゴリズムについて
        // 一切の保証をしていないため、将来的に変更される可能性がある点にご注意ください。
        assert_eq!(v.capacity(), 4);
    }
}
