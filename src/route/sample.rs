use axum::{
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    app_state::AppState,
    diag::{self, AppError},
};

async fn hello_axum() -> &'static str {
    //tracing::debug!("hello_axum");
    "Hello Axum"
}

async fn get_state(State(state): State<AppState>) -> String {
    let data = state.data.lock().expect("poisend");
    data.to_owned()
}

#[derive(Deserialize, Debug)]
pub struct StateInput {
    data: String,
}

async fn post_state(
    State(state): State<AppState>,
    // 1 Json(payload): Json<StateInput> or Json(payload) : Json<Value>
    // 2 WithRejection(Json(payload), _) : WithRejection<Json<StateInput>, diag::AppError>
    // 3 payload: Result<Json<StateInput>, JsonRejection>,
    Json(payload) : Json<Value>
) -> impl IntoResponse {
    if let Some(data) = payload.get("data") {
        print!("{}", data);
    }
    
    // match payload {
    //     Ok(payload) => {
    //         let mut data = state.data.lock().unwrap();
    //         *data = payload.0.data;
    //         StatusCode::ACCEPTED.into_response()
    //     }
    //     Err(err) => AppError::JsonRejection(err).into_response(),
    // }
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(hello_axum))
        .route("/state", get(get_state).post(post_state))
}
