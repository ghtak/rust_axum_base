use async_session::SessionStore;
use axum::{async_trait, extract::{FromRef, FromRequestParts}};
use http::request::Parts;
use serde::{Deserialize, Serialize};

use crate::{depends::Depends, diag::{self, AppError}};

use super::{extract::session_from_parts, StoreImpl};

#[derive(Serialize, Deserialize)]
pub struct User {
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
