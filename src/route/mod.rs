use axum::Router;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{app_state::AppState, session::session_route};

mod sample;

pub(crate) fn init(app_state: AppState) -> anyhow::Result<Router> {
    let route = Router::new()
        .merge(sample::router())
        .merge(session_route())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())   
        )
        .with_state(app_state);
    Ok(route)
}

