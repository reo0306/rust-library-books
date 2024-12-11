use std::sync::Arc;

use adapter::{
    database::ConnectionPool,
    redis::RedisClient,
    repository::{
        book::BookRepositoryImpl,
        health::HealthCheckRepositoryImpl,
        auth::AuthRepositoryImpl,
        user::UserRepositoryImpl,
    },
};
use kernel::repository::{
    book::BookRepository,
    health::HealthCheckRepository,
    auth::AuthRepository,
    user::UserRepository,
};
use shared::config::AppConfig;

// 1) DIコンテナの役割を果たす構造体を定義する。Cloneはのちほどaxum側で必要になるため
#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthCheckRepository>,
    book_repository: Arc<dyn BookRepository>,
    auth_repository: Arc<dyn AuthRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool, redis_client: Arc<RedisClient>, app_config: AppConfig) -> Self {
        // 2) 依存解決を行う。関数内で手書きする。
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let book_repository = Arc::new(BookRepositoryImpl::new(pool.clone()));
        let auth_repository = Arc::new(AuthRepositoryImpl::new(pool.clone(), redis_client.clone(), app_config.auth.ttl));
        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));

        Self {
            health_check_repository,
            book_repository,
            auth_repository,
            user_repository,
        }
    }

    // 3) 依存解決したインスタンスを返すメソッドを定義する
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    pub fn book_repository(&self) -> Arc<dyn BookRepository> {
        self.book_repository.clone()
    }

    pub fn auth_repository(&self) -> Arc<dyn AuthRepository> {
        self.auth_repository.clone()
    }

    pub fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }
}
