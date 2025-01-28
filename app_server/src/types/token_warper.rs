use axum::{extract::FromRequestParts, Json};
use axum_extra::extract::CookieJar;
use domain::entity::token::Token;
use http::StatusCode;

use crate::COOKIE_KEY;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TokenWrap(pub Token);

impl<S> FromRequestParts<S> for TokenWrap
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<crate::error::Error>);

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut axum::http::request::Parts,
        state: &'life1 S,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self, Self::Rejection>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
        let err = (StatusCode::UNAUTHORIZED, Json(crate::error::Error::new(0, "token not found")));

        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| err.clone())?;

        match jar.get(COOKIE_KEY) {
                Some(val) => {
                    let token = val.value_trimmed();
                    Ok(TokenWrap(Token(token.to_owned())))
                }
                None => Err(err),
            }
        })
    }
}
