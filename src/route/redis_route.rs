use axum::{extract::Path, routing::get, Router};
use axum_extra::extract::WithRejection;

use crate::{app_state::AppState, diag, redis::RedisConnection};

async fn redis_set(
    RedisConnection(mut conn): RedisConnection,
    WithRejection(Path((key, value)), _): WithRejection<Path<(String, String)>, diag::AppError>,
) -> diag::Result<()> {
    bb8_redis::redis::cmd("SET")
        .arg(key)
        .arg(value)
        .query_async(&mut *conn)
        .await?;
    Ok(())
}

async fn redis_get(
    RedisConnection(mut conn): RedisConnection,
    WithRejection(Path(key), _): WithRejection<Path<String>, diag::AppError>,
) -> diag::Result<String> {
    let value: String = bb8_redis::redis::cmd("GET")
        .arg(key)
        .query_async(&mut *conn)
        .await
        .unwrap();
    Ok(value)
}

async fn redis_ping(RedisConnection(mut conn): RedisConnection) -> diag::Result<String> {
    let replay: String = bb8_redis::redis::cmd("PING")
        .query_async(&mut *conn)
        .await
        .unwrap();
    Ok(replay)
}

pub(crate) fn redis_route() -> Router<AppState> {
    Router::new()
        .route("/redis/:key/:value", get(redis_set))
        .route("/redis/:key", get(redis_get))
        .route("/redis/ping", get(redis_ping))
}
