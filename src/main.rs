use tokio::net::TcpListener;

use crate::config::Config;

mod config;
mod diag;
mod route;

#[tokio::main]
async fn main() {
    let config = Config::new("./app_config.toml").unwrap();
    let _guard = config.tracing.init().unwrap();
    let listener = TcpListener::bind(config.http.socketaddr().unwrap())
        .await
        .unwrap();
    let route = route::init().unwrap();
    axum::serve(listener, route).await.unwrap();
}
