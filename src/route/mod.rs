use axum::Router;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod sample;

pub(crate) fn init() -> anyhow::Result<Router> {
    let route = Router::new()
        .merge(sample::router())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())   
        );
    Ok(route)
}

