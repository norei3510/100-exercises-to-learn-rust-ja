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

    // 以下の数値はすべて、64ビット環境で動作していることを前提としています。
    // 32ビット環境で動作している場合は、ポインタのサイズは8バイトではなく4バイトになります。

    #[test]
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), 8);
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&mut u64>(), 8);
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), 8);
    }
}
