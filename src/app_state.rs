use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub(crate) data: Arc<Mutex<String>>,
}

impl AppState {
    pub(crate) fn new() -> Self {
        AppState {
            data: Arc::new(Mutex::new("Data".to_owned())),
        }
    }
}
