use async_trait::async_trait;
use derive_new::new;
use kernel::repository::health::HealthCheckRepository;

use crate::database::ConnectionPool;

// 1) コンストラクトを生成する
#[derive(new)]
pub struct HealthCheckRepositoryImpl {
    // 2) 構造体にConnectionPoolをもたせる。
    db: ConnectionPool,
}

#[async_trait]
// 3) HealthCheckRepositoryを実装する
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn check_db(&self) -> bool {
        // 4) クエリ実行結果はResult型であるため、OKならtrue, Errならfalseを返される
        sqlx::query("SELECT 1")
            .fetch_one(self.db.inner_ref())
            .await
            .is_ok()
    }
}
