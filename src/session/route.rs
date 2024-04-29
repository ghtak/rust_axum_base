use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use http::{HeaderMap, StatusCode};

use crate::{
    app_state::AppState,
    depends::Depends,
    diag::{self, AppError},
};

use async_session::Session;

use super::sample_user::{User, USERKEY};


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
