use crate::{TicketDescription, TicketTitle};
use common::{valid_description, valid_title};

/// テスト用の、
/// 有効なチケットタイトルを生成する関数です。
pub fn ticket_title() -> TicketTitle {
    valid_title().try_into().unwrap()
}

/// テスト用の、有効なチケット説明文を生成する関数です。
pub fn ticket_description() -> TicketDescription {
    valid_description().try_into().unwrap()
}
