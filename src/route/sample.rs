use std::collections::HashMap;

use axum::{
    body::Bytes,
    extract::{rejection::JsonRejection, Path, Query, Request, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    app_state::AppState,
    diag::{self, AppError},
};

async fn hello_axum() -> &'static str {
    "Hello Axum"
}

async fn get_state(State(state): State<AppState>) -> String {
    let data = state.data.lock().await;
    data.to_owned()
}

#[derive(Deserialize, Debug)]
pub struct StateInput {
    data: String,
}

async fn post_state_result(
    State(state): State<AppState>,
    payload: Result<Json<StateInput>, JsonRejection>,
) -> impl IntoResponse {
    // https://docs.rs/axum/latest/axum/extract/index.html
    match payload {
        Ok(payload) => {
            let mut data = state.data.lock().await;
            *data = payload.0.data;
            StatusCode::ACCEPTED.into_response()
        }
        Err(err) => {
            AppError::JsonRejection(err).into_response()
        }
    }
}

async fn post_state_json_rejection(
    State(state): State<AppState>,
    WithRejection(Json(payload), _): WithRejection<Json<StateInput>, diag::AppError>,
) -> impl IntoResponse {
    let mut data = state.data.lock().await;
    *data = payload.data;
    StatusCode::ACCEPTED.into_response()
}

async fn post_state_json_value(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    if let Some(data) = payload.get("data") {
        let mut state_data = state.data.lock().await;
        *state_data = data.as_str().unwrap().to_owned();
        StatusCode::ACCEPTED.into_response()
    } else {
        AppError::Unknown("payload parse error".to_owned()).into_response()
    }
}

async fn post_state_json_type(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    if let Some(data) = payload.get("data") {
        let mut state_data = state.data.lock().await;
        *state_data = data.as_str().unwrap().to_owned();
        StatusCode::ACCEPTED.into_response()
    } else {
        AppError::Unknown("payload parse error".to_owned()).into_response()
    }
}

//https://docs.rs/axum/latest/axum/extract/index.html
async fn path(Path(_user_id): Path<i32>) {}
async fn query(Query(_params): Query<HashMap<String, String>>) {}
async fn headers(_headers: HeaderMap) {}
async fn string(_body: String) {}
async fn bytes(_body: Bytes) {}
async fn request(_request: Request) {}
async fn extension(Extension(_state): Extension<AppState>) {}

#[derive(Deserialize, Debug)]
struct Pagination {
    page: usize,
    per_page: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 10,
        }
    }
}

async fn get_with_pagenation(Path(user_id): Path<String>, pagination: Option<Query<Pagination>>) {
    let Query(pagination) = pagination.unwrap_or_default();
    tracing::debug!("{} {:?}", user_id, pagination);
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(hello_axum))
        .route("/state", get(get_state).post(post_state_json_rejection))
        .route("/path/:user_id", post(path))
        .route("/query", post(query))
        .route("/string", post(string))
        .route("/bytes", post(bytes))
        .route("/request", post(request))
        .route("/extension", post(extension))
        .route("/get_with_pagenation/:user_id", get(get_with_pagenation))
}
