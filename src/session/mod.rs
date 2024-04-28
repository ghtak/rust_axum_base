pub mod exchange;
mod sample_user;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use http::{request::Parts, HeaderMap, HeaderName, StatusCode};

use crate::{
    app_state::AppState,
    depends::Depends,
    diag::{self, AppError},
};

use async_session::{Session, SessionStore};

use self::{
    exchange::SessionExchange,
    sample_user::{User, USERKEY},
};

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

pub(crate) type ContextType = Context<async_session::MemoryStore, exchange::CookieExchange>;
pub(crate) const SESSIONID: &'static str = "SESSIONID";

impl FromRef<AppState> for ContextType {
    fn from_ref(input: &AppState) -> Self {
        input.session_context.clone()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Depends<async_session::Session>
where
    S: Send + Sync,
    ContextType: FromRef<S>,
{
    type Rejection = diag::AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let context = ContextType::from_ref(state);
        let session = context.load(&parts.headers).await?;
        Ok(Depends(session))
    }
}

async fn login(State(state): State<AppState>) -> diag::Result<impl IntoResponse> {
    let user = User::new(0, "testUser");
    let mut session = Session::new();
    session
        .insert(USERKEY, &user)
        .map_err(|e| AppError::Unknown(e.to_string()))?;

    let resp_header = state.session_context.store(session).await?;

    Ok((StatusCode::OK, resp_header))
}

async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> diag::Result<impl IntoResponse> {
    let session = state.session_context.load(&headers).await?;
    let resp = state.session_context.destroy(session).await?;
    if let Some(resp_header) = resp {
        Ok((StatusCode::OK, resp_header).into_response())
    } else {
        Ok(StatusCode::OK.into_response())
    }
}

async fn get_user(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> diag::Result<impl IntoResponse> {
    let session = state.session_context.load(&headers).await?;
    let user = session
        .get::<User>(USERKEY)
        .ok_or(AppError::InvalidSession)?;

    Ok((StatusCode::OK, Json(user)).into_response())
}

async fn get_user_s(Depends(session): Depends<Session>) -> diag::Result<impl IntoResponse> {
    let user = session
        .get::<User>(USERKEY)
        .ok_or(AppError::InvalidSession)?;

    Ok((StatusCode::OK, Json(user)).into_response())
}

async fn get_user_d(Depends(user): Depends<User>) -> diag::Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(user)).into_response())
}

pub(crate) fn context_route() -> Router<AppState> {
    Router::new()
        .route("/context/login", get(login))
        .route("/context/logout", get(logout))
        .route("/context/user", get(get_user))
        .route("/context/user_s", get(get_user_s))
        .route("/context/user_d", get(get_user_d))
}
