use async_session::Session;
use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use http::header::SET_COOKIE;
use reqwest::StatusCode;

use crate::{
    app_state::AppState,
    depends::Depends,
    diag::{self, AppError},
};

use super::{
    sample_user::{User, USERKEY},
    SESSIONID,
};

// async fn login(
//     State(state): State<AppState>,
// ) -> diag::Result<impl IntoResponse> {
//     let user = User::new(0, "testUser");
//     let mut session = Session::new();
//     session
//         .insert(USERKEY, &user)
//         .map_err(|e| AppError::Unknown(e.to_string()))?;

//     let cookie_value = state.store_session(session).await?;

//     let mut cookie = Cookie::new(SESSIONID, cookie_value);
//     cookie.set_same_site(SameSite::Lax);
//     cookie.set_http_only(true);

//     let res =  [(SET_COOKIE, cookie.to_string())];
//     Ok((StatusCode::OK, 
//         [(SET_COOKIE, cookie.to_string())]
//         , Json(user)))
// }

// async fn logout(
//     State(state): State<AppState>,
//     cookiejar: CookieJar,
// ) -> diag::Result<impl IntoResponse> {
//     let cookie = cookiejar.get(SESSIONID).ok_or(AppError::NoSession)?;
//     let _res = state.destroy_session(cookie).await;
//     let jar = cookiejar.remove(Cookie::from(SESSIONID));

//     Ok((StatusCode::OK, jar).into_response())
// }

// async fn get_user(
//     State(state): State<AppState>,
//     cookiejar: CookieJar,
// ) -> diag::Result<impl IntoResponse> {
//     let cookie = cookiejar.get(SESSIONID).ok_or(AppError::NoSession)?;
//     let session = state.load_session(cookie).await?;
//     let user = session
//         .get::<User>("user")
//         .ok_or(AppError::InvalidSession)?;

//     Ok((StatusCode::OK, Json(user)).into_response())
// }

// async fn get_user_extract(Depends(session): Depends<Session>) -> diag::Result<impl IntoResponse> {
//     let user = session
//         .get::<User>(USERKEY)
//         .ok_or(AppError::InvalidSession)?;

//     Ok((StatusCode::OK, Json(user)).into_response())
// }

// async fn get_user_depends(Depends(user): Depends<User>) -> diag::Result<impl IntoResponse> {
//     Ok((StatusCode::OK, Json(user)).into_response())
// }

// pub(crate) fn session_route() -> Router<AppState> {
//     Router::new()
//         .route("/session/login", get(login))
//         .route("/session/user", get(get_user))
//         .route("/session/user/ext", get(get_user_extract))
//         .route("/session/user/dep", get(get_user_depends))
//         .route("/session/logout", get(logout))
// }
