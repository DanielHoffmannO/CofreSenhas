use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

/// Erros específicos de HTTP/domínio da API web -- distintos de
/// `crate::error::VaultError`, que é da CLI. Cada variante já sabe em que
/// status code vira, então os handlers só precisam usar `?`.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    BadRequest(String),

    #[error("credenciais inválidas")]
    Unauthorized,

    #[error("recurso não encontrado")]
    NotFound,

    #[error("{0}")]
    Conflict(String),

    #[error("erro interno: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };
        (status, Json(json!({ "message": message }))).into_response()
    }
}

// Qualquer coisa que já usa `crate::error::VaultError` (crypto, storage)
// vira automaticamente um erro interno de servidor via `?`, sem cada
// handler precisar fazer o `match` manualmente.
impl From<crate::error::VaultError> for AppError {
    fn from(err: crate::error::VaultError) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<r2d2::Error> for AppError {
    fn from(err: r2d2::Error) -> Self {
        AppError::Internal(format!("pool de conexões: {err}"))
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        match err {
            rusqlite::Error::QueryReturnedNoRows => AppError::NotFound,
            other => AppError::Internal(other.to_string()),
        }
    }
}
