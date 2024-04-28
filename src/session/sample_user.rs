use axum::{async_trait, extract::{FromRef, FromRequestParts}};
use http::request::Parts;
use serde::{Deserialize, Serialize};

use crate::{depends::Depends, diag::{self, AppError}};

use super::ContextType;


pub const USERKEY : &'static str = "user";

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
    ContextType: FromRef<S>,
{
    type Rejection = diag::AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let context = ContextType::from_ref(state);
        let session = context.load(&parts.headers).await?;
        let user = session
            .get::<User>(USERKEY)
            .ok_or(AppError::InvalidSession)?;
        Ok(Depends(user))
    }
}
