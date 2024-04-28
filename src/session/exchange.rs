use axum_extra::extract::cookie::{Cookie, SameSite};
use cookie::time::OffsetDateTime;
use http::{
    header::{COOKIE, SET_COOKIE},
    HeaderMap, HeaderName,
};
use std::str::FromStr;

use crate::diag::{self, AppError};

pub(crate) trait SessionExchange {
    fn build(
        &self,
        sessionid: &'_ str,
        session_key: &'_ str,
    ) -> diag::Result<[(HeaderName, String); 1]>;

    fn extract(&self, sessionid: &'_ str, headers: &HeaderMap) -> diag::Result<String>;

    fn destory(&self, sessionid: &'_ str) -> diag::Result<Option<[(HeaderName, String); 1]>>;
}

#[derive(Default, Clone, Debug)]
pub(crate) struct CookieExchange {}

impl CookieExchange {
    pub(crate) fn new() -> Self {
        CookieExchange::default()
    }
}

impl SessionExchange for CookieExchange {
    fn build(
        &self,
        sessionid: &'_ str,
        session_key: &'_ str,
    ) -> diag::Result<[(HeaderName, String); 1]> {
        let mut cookie = Cookie::new(sessionid, session_key);
        cookie.set_same_site(SameSite::Lax);
        cookie.set_http_only(true);
        Ok([(SET_COOKIE, cookie.to_string())])
    }

    fn extract(&self, sessionid: &'_ str, headers: &HeaderMap) -> diag::Result<String> {
        let cookies = headers
            .get_all(COOKIE)
            .into_iter()
            .filter_map(|value| value.to_str().ok())
            .flat_map(|value| value.split(';'))
            .filter_map(|cookie| Cookie::parse_encoded(cookie.to_owned()).ok());

        for c in cookies {
            if c.name().eq(sessionid) {
                return Ok(c.value().to_owned());
            }
        }

        Err(AppError::NoSession)
    }

    fn destory(&self, sessionid: &'_ str) -> diag::Result<Option<[(HeaderName, String); 1]>> {
        let mut cookie = Cookie::new(sessionid, "");
        cookie.set_expires(OffsetDateTime::UNIX_EPOCH);
        Ok(Some([(SET_COOKIE, cookie.to_string())]))
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct HeaderExchange {}

impl HeaderExchange {
    pub(crate) fn new() -> Self {
        HeaderExchange::default()
    }
}

impl SessionExchange for HeaderExchange {
    fn build(
        &self,
        sessionid: &'_ str,
        session_key: &'_ str,
    ) -> diag::Result<[(HeaderName, String); 1]> {
        let header = HeaderName::from_str(sessionid)
            .map_err(|_e| AppError::Unknown("Invalid Header Key".to_owned()))?;
        Ok([(header, session_key.to_string())])
    }

    fn extract(&self, sessionid: &'_ str, headers: &HeaderMap) -> diag::Result<String> {
        let value = headers
            .get(sessionid)
            .ok_or(AppError::NoSession)?
            .to_str()
            .map_err(|err| AppError::Unknown(err.to_string()))?;
        Ok(value.to_owned())
    }

    fn destory(&self, _sessionid: &'_ str) -> diag::Result<Option<[(HeaderName, String); 1]>> {
        Ok(None)
    }
}
