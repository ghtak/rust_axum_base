use async_session::{Session, SessionStore};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::extract::cookie::{self, Cookie};
use http::header::COOKIE;

use crate::{
    app_state::AppState,
    depends::Depends,
    diag::{self, AppError},
};

use super::{StoreImpl, SESSIONID};

impl FromRef<AppState> for StoreImpl {
    fn from_ref(input: &AppState) -> Self {
        input.session_store.clone()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Depends<async_session::Session>
where
    S: Send + Sync,
    StoreImpl: FromRef<S> + SessionStore,
{
    type Rejection = diag::AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .headers
            .get_all(COOKIE)
            .into_iter()
            .filter_map(|value| value.to_str().ok())
            .flat_map(|value| value.split(';'))
            .filter_map(|cookie| Cookie::parse_encoded(cookie.to_owned()).ok());

        for c in cookies {
            if c.name() == SESSIONID {
                let store = StoreImpl::from_ref(state);
                let session = store
                    .load_session(c.value().to_owned())
                    .await
                    .map_err(|e| AppError::Unknown(e.to_string()))?
                    .ok_or(AppError::InvalidSession)?;
                    
                return Ok(Depends(session))
            }
        }

        Err(AppError::NoSession)
    }
}
