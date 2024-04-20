use axum::{routing::get, Router};

async fn hello_axum() -> &'static str {
    //tracing::debug!("hello_axum");
    "Hello Axum"
}


pub(crate) fn router() -> Router {
    Router::new()
        .route("/", get(hello_axum))
}