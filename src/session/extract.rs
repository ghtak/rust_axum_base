use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
};
use http::request::Parts;

use crate::{
    app_state::AppState,
    depends::Depends,
    diag::{self},
};

use super::ContextType;

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