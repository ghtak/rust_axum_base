#![allow(dead_code)]

use app_state::AppState;
use config::Config;
use tokio::net::TcpListener;

mod app_state;
mod config;
mod diag;
mod route;
mod tests;

#[tokio::main]
async fn main() {
    let config = Config::new("./app_config.toml").unwrap();
    let _guard = config.tracing.init().unwrap();
    let app_state = AppState::new();

    let listener = TcpListener::bind(config.http.socketaddr().unwrap())
        .await
        .unwrap();

    let route = route::init(app_state).unwrap();
    axum::serve(listener, route).await.unwrap();
}
