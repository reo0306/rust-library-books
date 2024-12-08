use std::env;
use strum::EnumString;

#[derive(Default, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    // 開発環境向けで動作していることを示す
    #[default]
    Development,
    // 本番環境向けで動作していることを示す
    Production,
}

// 開発環境、本番環境のどちら向けのビルドであるかを示す
pub fn which() -> Environment {
    // debug_assertionsがonの場合はデバッグビルド
    // そうでない場合はリリースビルドだと判定する。
    // 以下の let default_env = 〜は片方だけが実行される
    #[cfg(debug_assertions)]
    let default_env = Environment::Development;
    #[cfg(not(debug_assertions))]
    let default_env = Environment::Production;

    match env::var("ENV") {
        Err(_) => default_env,
        Ok(v) => v.parse().unwrap_or(default_env),
    }
}
