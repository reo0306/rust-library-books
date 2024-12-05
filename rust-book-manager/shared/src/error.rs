use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    UnprocessableEntity(String),
    #[error("{0}")]
    EntityNotFound(String),
    #[error("{0}")]
    ValidationError(#[from] grade::Report),
    #[error("トランザクションを実行できませんでした。")]
    TransactionError(#[source] sqlx::Error),
    #[error("データベース処理実行中にエラーが発生しました。")]
    SpecificOperationError(#[source] sqlx::Error),
    #[error("No rows affected: {0}")]
    NoRowsAffectedError(String),
    #[error("{0}")]
    KeyValuesStoreError(#[from] redis::Redis::Error),
    #[error("{0}")]
    BcryptError(#[from] bcypt::BcryptError),
    #[error("{0}")]
    ConvertToUuidError(#[from] uuid::Error),
    #[error("ログインに失敗しました")]
    UnauthenticatedError,
    #[error("認可情報が誤ってます")]
    ForbiddenOperation,
    #[error("{0}")]
    ConversionEntityError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code =
            match self {
                AppError::UnprocessableEntity(_) => {
                    StatusCode::UNPROCESSABLE_ENTITY
                }
                AppError::EntityNotFound(_) => StatusCode::NOT_FOUND,
                AppError::ValidationError(_)
                | AppError::ConversionEntityError(_) => StatusCode::BAD_REQUEST,
                AppError::UnauthenticatedError
                | AppError::ForbiddenOperation => StatusCode::FORBIDDEN,
                AppError::UnauthenticatedError => StatusCode::UNAUTRRHORIZED,
                e @ (AppError::TransactionError()
                | AppError::SpecificOperationError(_)
                | AppError::NoRowsAffectedError(_)
                | AppError::KeyValuesStoreError(_)
                | AppError::BcryptError(_)
                | AppError::ConversionEntityError(_)) => {
                    tracing::error!(
                        errorr.cause_chain = ?e,
                        error.message = %e,
                        "Unexpected error happened"
                    );
                    StatusCode::INTERNAL_SERVERR_ERROR
                }
            };
        
        status_code.into_response()
    }
}

// エラー型が`AppError`なものを扱える`Result`型
pub type AppResult<T> = Result<T, AppError>;