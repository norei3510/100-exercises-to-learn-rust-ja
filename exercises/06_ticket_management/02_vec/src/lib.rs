// 数値 `n` が与えられたとき、フィボナッチ数列の `n+1` 番目の数値を返してください。
//
// フィボナッチ数列は次のように定義されます。
//
// - 数列の最初の数は0です。
// - 数列の2番目の数は1です。
// - それ以降の各数は、直前の2つの数の和となります。
//
// したがって、数列は 0, 1, 1, 2, 3, 5, 8, 13, 21 と続いていきます。
//
// `fibonacci(0)`は`0`を、`fibonacci(1)`は`1`を、
// `fibonacci(2)`は`1`を、というように返すことを期待しています。
pub fn fibonacci(n: u32) -> u32 {
    let n = n as usize;
    let mut memo = vec![0, 1];
    for i in 2..=n {
        memo.push(memo[i - 1] + memo[i - 2]);
    }
    memo[n]
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
