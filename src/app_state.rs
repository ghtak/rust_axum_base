use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    database::{self, DatabasePoolType},
    redis, session,
};

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub(crate) data: Arc<Mutex<String>>,
    pub(crate) session_context: session::ContextType,
    pub(crate) db_pool: database::DatabasePoolType,
    pub(crate) redis_pool: redis::RedisPoolType,
}

impl AppState {
    pub(crate) fn new(db_pool: DatabasePoolType, redis_pool: redis::RedisPoolType) -> Self {
        AppState {
            data: Arc::new(Mutex::new("Data".to_owned())),
            session_context: session::ContextType::new(
                session::SESSIONID.to_owned(),
                async_session::MemoryStore::new(),
            ),
            db_pool,
            redis_pool,
        }
    }
}
