use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use adapter::{database::connect_database_with, redis::RedisClient};
use anyhow::{Context, Error, Result};
use api::route::{auth, v1};
use axum::Router;
use registry::AppRegistry;
use shared::{
    config::AppConfig,
    env::{which, Environment},
};
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // ロガーを初期化する関数init_loggerの呼び出しを追加
    init_logger()?;
    bootstrap().await
}

fn init_logger() -> Result<()> {
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    // ログレベルを設定
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    // ログの出力形式を設定
    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}

// 1) 後々ログの初期化など他の関数をmain関数内に挟むため、いまのうちにサーバー起動分だけ分離しておく。
async fn bootstrap() -> Result<()> {
    // 2) AppConfigを生成させる。
    let app_config = AppConfig::new()?;
    // 3) データベースへの接続を行う。コネクションプールを取り出しておく
    let pool = connect_database_with(&app_config.database);
    // Redisへの接続を行うクライアントのインスタンスを作成する。
    let kv = Arc::new(RedisClient::new(&app_config.redis)?);
    // 4) AppResitryを生成する
    let registry = AppRegistry::new(pool, kv, app_config);

    // 5) build_health_check_routers関数を呼び出す。AppRegistryをRouterに登録しておく
    let app = Router::new()
        .merge(v1::routes())
        .merge(auth::routes())
        // 以下に、リクエストとレスポンス時にログを出力するレイヤーを追加する
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .with_state(registry);

    // 6) サーバーを起動する。起動時と起動失敗時にログを設定する
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    // println!からAtrracing::info!に変更
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app)
        .await
        .context("Unexpected error happened in server")
        // 起動失敗した際のエラーログを tracing::error!で出力
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "Unexpected error"
            )
        })
}
