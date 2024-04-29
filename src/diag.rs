use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

pub(crate) type Result<T> = core::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub(crate) enum AppError {

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error("unknown error {0}")]
    Unknown(String),

    #[error("no session")]
    NoSession,

    #[error("invalid session")]
    InvalidSession,

    #[error(transparent)]
    SqlXError(sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            AppError::Unknown(strerr) => {
                (StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "message": format!("{}", strerr) }))).into_response()
            },
            AppError::JsonRejection(err) => {
                (StatusCode::BAD_REQUEST,
                    Json(json!({ "message": format!("{:?}", err) }))).into_response()
            },
            err => {
                (StatusCode::BAD_REQUEST, err.to_string()).into_response()
            }
        };
        resp
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::SqlXError(value)
    }
}