use crate::status::Status;

// 最初の方の演習でモジュールの宣言方法を学びましたが、
// それらを別ファイルに切り出す方法はまだ見ていません。
// さっそくやってみましょう！
//
// 最も単純なケースとして、抽出するモジュールが単一のファイルである場合は、
// モジュールと同じ名前の新しいファイルを作成し、そこにモジュールの内容を移動すれば十分です。
// モジュールファイルは、モジュールを宣言しているファイルと同じディレクトリに配置してください。
// この場合、`src/lib.rs`、したがって`status.rs`は`src`ディレクトリに配置する必要があります。
mod status;

// TODO: ステータス文字列が無効な場合に備えて、`TicketNewError`に新しいエラーバリアントを追加してください。
//   そのバリアントのエラーに対して`source`を呼び出した際は、`None`ではなく`ParseStatusError`を返すようにしてください。

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // TODO: ステータス文字列を`Status`列挙型にパースします。

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}
