use axum::{async_trait, extract::{FromRef, FromRequestParts}};
use http::request::Parts;

use crate::diag;

pub(crate) struct Usecase<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Usecase<T>
where
    S: Sync + Send,
    T: FromRef<S>,
{
    type Rejection = diag::AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Usecase::<T>(T::from_ref(state)))
    }
}

pub mod get_user_usecase;