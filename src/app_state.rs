use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    database::{self, PoolType},
    session,
};

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub(crate) data: Arc<Mutex<String>>,
    pub(crate) session_context: session::ContextType,
    pub(crate) db_pool: database::PoolType,
}

impl AppState {
    pub(crate) fn new(db_pool: PoolType) -> Self {
        AppState {
            data: Arc::new(Mutex::new("Data".to_owned())),
            session_context: session::ContextType::new(
                session::SESSIONID.to_owned(),
                async_session::MemoryStore::new(),
            ),
            db_pool,
        }
    }
}
