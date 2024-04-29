use sqlx::{postgres::PgPoolOptions, PgPool};

pub(crate) type PoolType = PgPool;
pub(crate) type PoolOptionsType = PgPoolOptions;
