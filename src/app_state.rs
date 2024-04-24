use std::sync::Arc;

use async_session::{Session, SessionStore};
use axum_extra::extract::cookie::Cookie;
use tokio::sync::Mutex;

use crate::{
    diag::{self, AppError},
    session::StoreImpl,
};

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub(crate) data: Arc<Mutex<String>>,
    pub(crate) session_store: StoreImpl,
}

impl AppState {
    pub(crate) fn new() -> Self {
        AppState {
            data: Arc::new(Mutex::new("Data".to_owned())),
            session_store: StoreImpl::new(),
        }
    }

    pub async fn load_session(self: &Self, cookie: &Cookie<'_>) -> diag::Result<Session> {
        let session = self
            .session_store
            .load_session(cookie.value().to_string())
            .await
            .map_err(|err| AppError::Unknown(err.to_string()))?
            .ok_or(AppError::InvalidSession)?;
        Ok(session)
    }

    pub async fn save_session(self: &Self, session: Session) -> diag::Result<String> {
        let cookie_value = self
            .session_store
            .store_session(session)
            .await
            .map_err(|e| AppError::Unknown(e.to_string()))?
            .unwrap();
        Ok(cookie_value)
    }

    pub async fn destroy_session(self: &Self, cookie: &Cookie<'_>) -> diag::Result<()> {
        let session = self.load_session(cookie).await?;
        self.session_store
            .destroy_session(session)
            .await
            .map_err(|err| AppError::Unknown(err.to_string()))?;
        Ok(())
    }
}
