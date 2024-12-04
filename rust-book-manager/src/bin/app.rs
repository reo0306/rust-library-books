use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use api::route::{
    health::build_healtth_check_routers,
    book::build_book_routers,
};
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

// 1) 後々ログの初期化など他の関数をmain関数内に挟むため、いまのうちにサーバー起動分だけ分離しておく。
async fn bootstrap() -> Result<()> {
    // 2) AppConfigを生成させる。
    let app_config = AppConfig::new()?;
    // 3) データベースへの接続を行う。コネクションプールを取り出しておく
    let pool = connect_database_with(&app_config.database);

    // 4) AppResitryを生成する
    let registry = AppRegistry::new(pool);

    // 5) build_health_check_routers関数を呼び出す。AppRegistryをRouterに登録しておく
    let app = Router::new()
        .merge(build_healtth_check_routers())
        .merge(build_book_routers())
        .with_state(registry);

    // 6) サーバーを起動する
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)
}
