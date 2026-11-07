pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: このセクションで学んだ内容に基づき、`todo!()`を
//  各型に対応する正しい**スタックサイズ**に置き換えてください。
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // これは難しい問題ですね！
        // 「直感的な」答えが今回はたまたま正解となっていますが、
        // 一般的に、構造体のメモリレイアウトはより複雑なトピックです。
        // 気になる方は、The Rust Reference の「Type layout」セクションを
        // https://doc.rust-lang.org/reference/type-layout.html参照して詳細を確認してください。
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
