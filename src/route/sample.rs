use axum::{extract::State, routing::get, Router};

use crate::app_state::AppState;

async fn hello_axum() -> &'static str {
    //tracing::debug!("hello_axum");
    "Hello Axum"
}

async fn hello_state(State(state): State<AppState>) -> String {
    let data = state.data.lock().expect("poisend");
    data.to_owned()
}

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(hello_axum))
        .route("/state", get(hello_state))
}
