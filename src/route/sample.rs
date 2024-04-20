use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use crate::{app_state::AppState, diag};

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
    data: String
}

async fn post_state(
    State(state): State<AppState>,
    //Json(payload): Json<StateInput>
    WithRejection(Json(payload), _) : WithRejection<Json<StateInput>, diag::AppError>
) -> impl IntoResponse {
    //tracing::debug!("{:?}", payload);
    tracing::debug!("post_state");
    let mut data = state.data.lock().unwrap();
    *data = payload.data;
    StatusCode::ACCEPTED
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(hello_axum))
        .route("/state", get(get_state).post(post_state))
}
