use std::sync::Arc;

use adapter::{database::ConnectionPool, repository::health::HealthCheckRepositoryImpl};
use kernel::repository::health::HealthCheckRepository;

// 1) DIコンテナの役割を果たす構造体を定義する。Cloneはのちほどaxum側で必要になるため
#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthCheckRepository>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        // 2) 依存解決を行う。関数内で手書きする。
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));

        Self {
            health_check_repository,
        }
    }

    // 3) 依存解決したインスタンスを返すメソッドを定義する
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }
}
