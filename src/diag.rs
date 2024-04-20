use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

pub(crate) type Result<T> = core::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub(crate) enum AppError {
    #[error("unknown error")]
    Unknown,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            AppError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        };
        resp.into_response()
    }
}
