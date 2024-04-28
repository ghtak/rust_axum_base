use async_session::SessionStore;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::extract::cookie::Cookie;
use http::header::COOKIE;

use crate::{
    app_state::AppState,
    depends::Depends,
    diag::{self, AppError},
};

pub(crate) type StoreImpl = MemoryStore;

use super::SESSIONID;

impl FromRef<AppState> for StoreImpl {
    fn from_ref(input: &AppState) -> Self {
        input.session_store.clone()
    }
}

pub(crate) async fn session_from_cookie<S, StoreImpl>(
    parts: &mut Parts,
    state: &S,
) -> Result<Depends<async_session::Session>, diag::AppError>
where
    S: Send + Sync,
    StoreImpl: FromRef<S> + SessionStore,
{
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

            return Ok(Depends(session));
        }
    }

    Err(AppError::NoSession)
}

pub(crate) async fn session_from_header<S, StoreImpl>(
    parts: &mut Parts,
    state: &S,
) -> Result<Depends<async_session::Session>, diag::AppError>
where
    S: Send + Sync,
    StoreImpl: FromRef<S> + SessionStore,
{
    let value = parts
        .headers
        .get(SESSIONID)
        .ok_or(AppError::NoSession)?
        .to_str()
        .map_err(|err| AppError::Unknown(err.to_string()))?;

    let store = StoreImpl::from_ref(state);
    let session = store
        .load_session(value.to_owned())
        .await
        .map_err(|e| AppError::Unknown(e.to_string()))?
        .ok_or(AppError::InvalidSession)?;

    Ok(Depends(session))
}

// #[async_trait]
// impl<S> FromRequestParts<S> for Depends<async_session::Session>
// where
//     S: Send + Sync,
//     StoreImpl: FromRef<S> + SessionStore,
// {
//     type Rejection = diag::AppError;

//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         session_from_cookie::<S, StoreImpl>(parts, state).await
//     }
// }
