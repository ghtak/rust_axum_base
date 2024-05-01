use axum::{async_trait, extract::{FromRef, FromRequestParts}};
use bb8_redis::{bb8::{Pool, PooledConnection}, RedisConnectionManager};
use http::request::Parts;

use crate::{app_state::AppState, diag::{self, AppError}};


pub(crate) type RedisPoolType = Pool<RedisConnectionManager>;

pub(crate) struct RedisConnection(pub PooledConnection<'static, RedisConnectionManager>);

impl FromRef<AppState> for RedisPoolType{
    fn from_ref(input: &AppState) -> Self {
        input.redis_pool.clone()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RedisConnection
where
    S: Sync + Send,
    RedisPoolType: FromRef<S>
{
    type Rejection = diag::AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = RedisPoolType::from_ref(state);
        let conn = pool
            .get_owned()
            .await
            .map_err(|e| AppError::BB8Error(e.to_string()))?;
        Ok(Self(conn))
    }
}