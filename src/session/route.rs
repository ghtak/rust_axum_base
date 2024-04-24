use async_session::{Session, SessionStore};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use http::request::Parts;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    depends::Depends,
    diag::{self, AppError},
};

use super::{extract::session_from_parts, StoreImpl};

#[derive(Serialize, Deserialize)]
struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: i64, name: &'_ str) -> Self {
        User {
            id,
            name: name.to_owned(),
            email: "abd@mail.com".to_owned(),
        }
    }
}

async fn login(
    State(state): State<AppState>,
    cookiejar: CookieJar,
) -> diag::Result<impl IntoResponse> {
    let user = User::new(0, "testUser");
    let mut session = Session::new();
    session
        .insert("user", &user)
        .map_err(|e| AppError::Unknown(e.to_string()))?;

    let cookie = state
        .session_store
        .store_session(session)
        .await
        .map_err(|e| AppError::Unknown(e.to_string()))?
        .unwrap();

    let mut cookie = Cookie::new("SESSIONID", cookie);

    cookie.set_same_site(SameSite::Lax);
    cookie.set_http_only(true);

    let jar = cookiejar.add(cookie);

    Ok((StatusCode::OK, jar, Json(user)))
}

async fn logout(
    State(state): State<AppState>,
    cookiejar: CookieJar,
) -> diag::Result<impl IntoResponse> {
    if let Some(cookie) = cookiejar.get("SESSIONID") {
        let session = state
            .session_store
            .load_session(cookie.value().to_string())
            .await
            .map_err(|err| AppError::Unknown(err.to_string()))?
            .unwrap();

        let _ = state
            .session_store
            .destroy_session(session)
            .await
            .map_err(|err| AppError::Unknown(err.to_string()));

        let jar = cookiejar.remove(Cookie::new("SESSIONID", ""));

        Ok((StatusCode::OK, jar).into_response())
    } else {
        Err(AppError::NoSession)
    }
}

async fn get_user(
    State(state): State<AppState>,
    cookiejar: CookieJar,
) -> diag::Result<impl IntoResponse> {
    if let Some(cookie) = cookiejar.get("SESSIONID") {
        let session = state
            .session_store
            .load_session(cookie.value().to_string())
            .await
            .map_err(|err| AppError::Unknown(err.to_string()))?
            .ok_or(AppError::InvalidSession)?;

        let user = session
            .get::<User>("user")
            .ok_or(AppError::InvalidSession)?;

        Ok((StatusCode::OK, Json(user)).into_response())
    } else {
        Err(AppError::NoSession)
    }
}

async fn get_user_extract(Depends(session): Depends<Session>) -> diag::Result<impl IntoResponse> {
    let user = session
        .get::<User>("user")
        .ok_or(AppError::InvalidSession)?;

    Ok((StatusCode::OK, Json(user)).into_response())
}

#[async_trait]
impl<S> FromRequestParts<S> for Depends<User>
where
    S: Send + Sync,
    StoreImpl: FromRef<S> + SessionStore,
{
    type Rejection = diag::AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Depends(session) = session_from_parts::<S, StoreImpl>(parts, state).await?;
        let user = session
            .get::<User>("user")
            .ok_or(AppError::InvalidSession)?;
        Ok(Depends(user))
    }
}

async fn get_user_depends(Depends(user): Depends<User>) -> diag::Result<impl IntoResponse> {
    Ok((StatusCode::OK, Json(user)).into_response())
}

pub fn session_route() -> Router<AppState> {
    Router::new()
        .route("/session/login", get(login))
        .route("/session/user", get(get_user))
        .route("/session/user_ext", get(get_user_extract))
        .route("/session/user_dep", get(get_user_depends))
        .route("/session/logout", get(logout))
}
