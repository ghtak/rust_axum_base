use sqlx::{postgres::PgPoolOptions, PgPool};

pub(crate) type DatabasePoolType = PgPool;
pub(crate) type DatabasePoolOptionsType = PgPoolOptions;
