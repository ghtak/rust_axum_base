use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

use crate::config::Config;

mod config;

#[tokio::main]
async fn main() {
    let config = Config::new("./app_config.toml").unwrap();
    let addr = config.http.socketaddr().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();
    let route = Router::new().route("/", get(|| async { "Hello Axum" }));
    axum::serve(listener, route).await.unwrap();
}
