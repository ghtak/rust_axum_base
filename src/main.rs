#![allow(dead_code)]

use app_state::AppState;
use config::Config;
use tokio::net::TcpListener;

mod app_state;
mod config;
mod depends;
mod diag;
mod route;
mod session;
mod tests;
mod database;
mod entity;
mod repository;
mod usecase;

#[tokio::main]
async fn main() -> diag::Result<()> {
    let config = Config::new("./app_config.toml").unwrap();
    let _guard = config.tracing.init().unwrap();
    let db_pool = config.database.create_pool().await?;

    let app_state = AppState::new(db_pool);
    let listener = TcpListener::bind(config.http.socketaddr().unwrap())
        .await
        .unwrap();

    let route = route::init(app_state).unwrap();
    axum::serve(listener, route).await.unwrap();
    Ok(())
}
