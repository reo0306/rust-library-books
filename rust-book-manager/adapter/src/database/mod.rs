use shared::{
    config::DatabaseConfig,
    error::{AppError, AppResult},
};
use sqlx::{postgres::PgConnectOptions, PgPool};

pub mod model;

// 1) DatabaseConfigからPgConnectOptionsに変換する関数
fn make_pg_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

// 2) sqlx::PgPoolをラップする
#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    // 3) sqlx::PgPoolへの参照を取得する。
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }

    pub async fn begin(&self) -> AppResult<sqlx::Transaction<'_, sqlx::Postgres>> {
        self.0.begin().await.map_err(AppError::TransactionError)
    }
}

// 4) 返り値をConnectionPoolに変更し、内部実装もそれに合わせて修正した
pub fn connect_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(cfg)))
}
