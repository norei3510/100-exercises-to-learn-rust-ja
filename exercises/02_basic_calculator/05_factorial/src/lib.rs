// 非負整数 `n` を受け取り、
// `n` の階乗である `n!` を返す `factorial` という名前の関数を定義してください。
//
// `n`の階乗は、`n`までのすべての正の整数の積として定義されます。
// 例えば、`5!`（「5の階乗」と読みます）は`5 * 4 * 3 * 2 * 1`であり、`120`となります。
// `0!`は`1`と定義されます。
//
// `factorial(0)`は`1`を、`factorial(1)`は`1`を、
// `factorial(2)`は`2`を、というように返すことを期待しています。
//
// 学んだ内容だけを使ってください！まだループは使えませんので、再帰を使う必要があります！
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
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
