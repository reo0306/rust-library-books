use axum::{routing::get, Router};
use registry::AppRegistry;

use crate::handler::health::{health_check, health_check_db};

// 1) RouteのStateがAppResitryとなるため、Routerの型引数を指定する
pub fn build_healtth_check_routers() -> Router<AppRegistry> {
    // 2) ヘルスチェックに関連するパスのルートである/healthに個別のパスをネストする
    let routers = Router::new()
        .route("/", get(health_check))
        .route("/db", get(health_check_db));
    Router::new().nest("/health", routers)
}
