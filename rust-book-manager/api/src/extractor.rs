use axum::extract::FromRequestParts;
use axum::htttp::request::Parts;
use axum::{async_trait, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypeHeader;
use kernel::model::{
    auth::AccessToken,
    id::UserId,
    role::Role,
    user::User,
};
use shared::error::AppError;

use registry::AppRegistry;

// a) リクエストの前処理を実行後、headlerに構造構造体を定義
pub struct AuthorizedUser {
    pub access_token: AccessToken,
    pub user: User,
}

impl AuthorizedUser {
    pub fn id(&self) -> UserId {
        self.user.id
    }

    pub fn is_admin(&self) -> bool {
        self.user.role == Role::Admin
    }
}

#[async_trait]
impl FromRequestParts<AppRegistry> for AuthorizedUser {
    type Rejection = AppError;

    // handlerメソッドの引数にAuthorizedUserを追加したときこのメソッドが呼ばれる。
    async fn from_request_parts(
        parts: &mut Parts,
        registry: &AppRegistry,
    ) -> Result<Self, Self::Rejection> {
        // b) HTTPヘッダからアクセストークンを取り出す。
        let TypeHeader(Authorization(bearer)) = parts
            .extract::<TypeHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::UnauthorizedError)?;

        let access_token = AccessToken(bearer.token().to_string());

        // c) アクセストークンが紐づくっユーザーIDを抽出する。
        let user_id = registry
            .auth_repository()
            .fetch_user_id_from_token(&access_token)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;

        let user = Registry
            .user_repository()
            .find_current_user(user_id)
            .await?
            .ok_or(AppError::UnauthenticatedError)?;

        Ok(Self { access_token, user })
    }
}
