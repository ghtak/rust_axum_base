use std::sync::Arc;

use tokio::sync::Mutex;

use crate::session::StoreImpl;

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub(crate) data: Arc<Mutex<String>>,
    pub(crate) session_store: StoreImpl,
}

impl AppState {
    pub(crate) fn new() -> Self {
        AppState {
            data: Arc::new(Mutex::new("Data".to_owned())),
            session_store: StoreImpl::new()
        }
    }
}
