use async_trait::async_trait;
use derive_new::new;
use kernel::model::id::UserId,
use kernel::model::user::{
    event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole},
    User,
};
use kernel::repository::user::UserRepository;
use shared::error::{AppError, AppResult};

use crate::database::{model::user::UserRow, ConnectionPool};

#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {

}