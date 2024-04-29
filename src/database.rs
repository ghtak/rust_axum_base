use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{config::Config, diag::{self, AppError}};



pub(crate) async fn create_pool(config: &Config) -> diag::Result<Pool<Postgres>>{
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connection)
        .connect(&config.database.url)
        .await
        .map_err(|e| AppError::Unknown(e.to_string()))?;
    Ok(pool)
}