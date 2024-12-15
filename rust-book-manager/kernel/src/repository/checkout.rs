use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::{
    checkout::{
        event::{CreateCheckout, UpdateReturned},
        Checkout,
    },
    id::{BookId, UserId},
};

#[async_trait]
pub trait CheckoutRepository: Send + Sync {
    // 貸出操作を行う
    async fn create(&self, event: CreateCheckout) -> AppResult<()>;

    // 返却操作を行う
    async fn update_returned(&self, event: UpdateReturned) -> AppResult<()>;

    // すべての未返却の貸出し情報を取得する
    async fn find_unreturned_all(&self) -> AppResult<Vec<Checkout>>;

    // ユーザーIDに紐づく未返却の貸出し情報を取得する
    async fn find_unreturned_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Checkout>>;

    // 藏書の貸出履歴（返却済みも含む）を取得する
    async fn find_history_by_book_id(&self, book_id: BookId) -> AppResult<Vec<Checkout>>;
}