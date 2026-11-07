// TODO: `title` と `description` がアクセサメソッド経由で返される際は、
//   正規化（つまり、先頭と末尾の空白の除去）を行う必要があります。
//   Rustの標準ライブラリにこれに役立つメソッドがありますが、`String` の
//   ドキュメントには記載されていません。
//   それがどこで定義されており、どのように使用すればよいか分かりますか？

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        todo!()
    }

    pub fn description(&self) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
