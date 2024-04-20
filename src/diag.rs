use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

pub(crate) type Result<T> = core::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub(crate) enum AppError {

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error("unknown error")]
    Unknown,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            AppError::Unknown => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            AppError::JsonRejection(err) => {
                (StatusCode::BAD_REQUEST,
                    Json(json!({ "message": format!("{:?}", err) }))).into_response()
            }
        };
        resp
    }
}
