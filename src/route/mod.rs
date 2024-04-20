use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use self::sample::hello_axum;
mod sample;

pub(crate) fn init() -> anyhow::Result<Router> {
    let route = Router::new()
        .route("/", get(hello_axum))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())   
        );
    Ok(route)
}

