use kernel::model::{
    checkout::{Checkout, CheckoutBook},
    id::{BookId, CheckoutId, UserId},
};
use sqlx::types::chrono::{DateTime, Utc};

// 貸出状態を確認するための型
// 藏書が存在する場合はこの型にはまるレコードが存在し、その蔵書が貸し出しの場合は、checkout_idおよびuser_idがNoneではない値になる
// 蔵書が貸出中でない場合checkout_idもuser_idのNone
pub struct CheckoutStateRow {
    pub book_id: BookId,
    pub checkout_id: Option<CheckoutId>,
    pub user_id: UserId,
}

// 貸出中の一覧を取得する際に使う方
pub struct CheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<CheckoutRow> for Checkout {
    fn from(value: CheckoutRow) -> Self {
        let CheckoutRow{
            checkout_id,
            book_id,
            user_id,
            checked_out_at,
            title,
            author,
            isbn,
        } = value;
        Self {
            id: checkout_id,
            checked_out_by: user_id,
            checked_out_at,
            // 未返却なので、returuned_atはNoneを入れる
            returned_at: None,
            book: CheckoutBook {
                book_id,
                title,
                author,
                isbn,
            },
        }
    }
}

// 返却済みの貸出一覧を取得する際に使う型
pub struct ReturnedCheckoutRow {
    pub checkout_id: CheckoutId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub checked_out_at: DateTime<Utc>,
    pub returned_at: DateTime<Utc>,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl From<ReturnedCheckoutRow> for Checkout {
    fn from(value: ReturnedCheckoutRow) -> Self {
        let ReturnedCheckoutRow {
            checkout_id,
            book_id,
            user_id,
            checked_out_at,
            returned_at,
            title,
            author,
            isbn,
        } = value;
        Checkout {
            id: checkout_id,
            checked_out_by: user_id,
            checked_out_at,
            // 返却済みなので、returned_atには日時データが入る
            returned_at: Some(returned_at),
            book: CheckoutBook {
                book_id,
                title,
                author,
                isbn,
            },
        }
    }
}