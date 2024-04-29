use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
};
use http::request::Parts;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{app_state::AppState, diag};

pub(crate) type PoolType = PgPool;
pub(crate) type PoolOptionsType = PgPoolOptions;

impl FromRef<AppState> for PgPool {
    fn from_ref(input: &AppState) -> Self {
        input.db_pool.clone()
    }
}

mod sample_user_repository;
pub(crate) mod route;

pub(crate) struct Repository<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Repository<T>
where
    S: Sync + Send,
    T: FromRef<S>,
{
    type Rejection = diag::AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Repository::<T>(T::from_ref(state)))
    }
}
