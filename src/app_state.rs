use std::sync::Arc;

use tokio::sync::Mutex;

use crate::session;

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub(crate) data: Arc<Mutex<String>>,
    pub(crate) session_context: session::ContextType,
}

impl AppState {
    pub(crate) fn new() -> Self {
        AppState {
            data: Arc::new(Mutex::new("Data".to_owned())),
            session_context: session::ContextType::new(
                session::SESSIONID.to_owned(),
                async_session::MemoryStore::new(),
            ),
        }
    }
}
