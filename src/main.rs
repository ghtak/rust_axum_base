#![allow(dead_code)]

use app_state::AppState;
use config::Config;
use tokio::net::TcpListener;

mod app_state;
mod config;
mod database;
mod depends;
mod diag;
mod entity;
mod redis;
mod repository;
mod route;
mod session;
mod tests;
mod usecase;

#[tokio::main]
async fn main() -> diag::Result<()> {
    let config = Config::new("./app_config.toml").unwrap();
    let _guard = config.tracing.init().unwrap();
    let db_pool = config.database.create_pool().await?;
    let redis_pool = config.redis.create_pool().await?;

    let app_state = AppState::new(db_pool, redis_pool);
    let listener = TcpListener::bind(config.http.socketaddr().unwrap())
        .await
        .unwrap();

    let route = route::init(app_state).unwrap();
    axum::serve(listener, route).await.unwrap();
    Ok(())
}
