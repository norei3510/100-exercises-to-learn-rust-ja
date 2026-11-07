// 以下のフィールドを持つ`Order`という名前の構造体を定義してください。
// - `price`、符号なし整数
// - `quantity`、符号なし整数
//
// また、数量が0より大きい場合は`true`を、それ以外の場合は`false`を返す
// `is_available`という名前のメソッドも備えている必要があります。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
