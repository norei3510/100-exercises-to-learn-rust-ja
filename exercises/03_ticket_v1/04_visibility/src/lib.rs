mod ticket {
    struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }
    }
}

// TODO: **例外的に**、この演習では `ticket` モジュールと `tests` モジュールの両方を修正することになります。
#[cfg(test)]
mod tests {
    // TODO: 以下の use 文に関するコンパイラエラーを解消するため、親モジュールに必要な `pub` 修飾子を追加してください。
    use super::ticket::Ticket;

    // ただし、ご注意ください！use文をコンパイルできるように可視性を変更した後、
    // この関数がコンパイルできてしまうのは避けたいところです。
    // 実際にコンパイルできないことを確認したら、コメントアウトしてください。
    fn should_not_be_possible() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // この演習を実行しようとすると、次のようなエラーが表示されるはずです。
        //
        // error[E0616]: 構造体 `Ticket` のフィールド `description` はプライベートです
        //    |
        //    |              assert_eq!(ticket.description, "A description");
        //    |                         ^^^^^^^^^^^^^^^^^^
        //
        // TODO: 以下がコンパイルできないことを確認したら、
        //   その行をコメントアウトして次の演習に進んでください！
        assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // こちらも、前述と同様のエラーが発生し、不可能であるはずです。
        // （前のテストで問題のある行をコメントアウトした後にのみ、
        // コンパイルエラーが発生します ―― 次のコンパイル段階で！）
        //
        // これにより、`Ticket`インスタンスを取得する方法は`Ticket::new`のみであることが証明されます。
        // 不正なタイトルや説明文でチケットを作成することはできません！
        //
        // TODO: 以下がコンパイルできないことを確認したら、
        //   該当行をコメントアウトして次の演習に進んでください！
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: "To-Do".into(),
        };
    }
}
