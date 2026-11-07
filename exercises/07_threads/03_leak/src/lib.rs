// TODO: 整数のベクタが与えられたら、そのヒープアロケーションをリークさせてください。
//  その後、結果として得られた静的スライスを半分に分割し、
//  それぞれの半分を別々のスレッドで合計してください。
//  ヒント：`Vec::leak` を確認してみてください。

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let v = v.leak();
    let mid = v.len() / 2;
    let (v1, v2) = v.split_at(mid);

    let handle1 = thread::spawn(move || v1.into_iter().sum::<i32>());
    let handle2 = thread::spawn(move || v2.into_iter().sum::<i32>());

    handle1.join().unwrap() + handle2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
