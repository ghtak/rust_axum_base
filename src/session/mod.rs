mod exchange;
mod extract;
pub mod route;
mod sample_user;

use http::{HeaderMap, HeaderName};

use crate::diag::{self, AppError};

use async_session::{Session, SessionStore};

use self::exchange::SessionExchange;

#[derive(Clone, Debug)]
pub(crate) struct Context<Store, Exchange>
where
    Store: SessionStore,
    Exchange: SessionExchange + Default,
{
    sessionid: String,
    store: Store,
    exchange: Exchange,
}

impl<Store, Exchange> Context<Store, Exchange>
where
    Store: SessionStore,
    Exchange: SessionExchange + Default,
{
    pub(crate) fn new(sessionid: String, store: Store) -> Self {
        Context {
            sessionid,
            store,
            exchange: Exchange::default(),
        }
    }

    pub async fn store_raw(&self, session: Session) -> diag::Result<String> {
        let session_key = self
            .store
            .store_session(session)
            .await
            .map_err(|e| AppError::Unknown(e.to_string()))?
            .unwrap();
        Ok(session_key)
    }

    pub async fn load_raw(&self, session_key: &'_ str) -> diag::Result<Session> {
        let session = self
            .store
            .load_session(session_key.to_owned())
            .await
            .map_err(|e| AppError::Unknown(e.to_string()))?
            .ok_or(AppError::InvalidSession)?;
        Ok(session)
    }

    pub async fn destroy_raw(&self, session: Session) -> diag::Result<()> {
        self.store
            .destroy_session(session)
            .await
            .map_err(|err| AppError::Unknown(err.to_string()))?;
        Ok(())
    }

    pub async fn store(&self, session: Session) -> diag::Result<[(HeaderName, String); 1]> {
        let session_key = self
            .store
            .store_session(session)
            .await
            .map_err(|e| AppError::Unknown(e.to_string()))?
            .unwrap();

        self.exchange
            .build(self.sessionid.as_str(), session_key.as_str())
    }

    pub async fn load(&self, headers: &HeaderMap) -> diag::Result<Session> {
        let key = self.exchange.extract(&self.sessionid, headers)?;
        let session = self
            .store
            .load_session(key)
            .await
            .map_err(|e| AppError::Unknown(e.to_string()))?
            .ok_or(AppError::InvalidSession)?;
        Ok(session)
    }

    pub async fn destroy(
        &self,
        session: Session,
    ) -> diag::Result<Option<[(HeaderName, String); 1]>> {
        self.store
            .destroy_session(session)
            .await
            .map_err(|err| AppError::Unknown(err.to_string()))?;
        self.exchange.destory(&self.sessionid)
    }
}

pub(crate) const SESSIONID: &'static str = "SESSIONID";
pub(crate) type ContextType = Context<async_session::MemoryStore, exchange::CookieExchange>;
