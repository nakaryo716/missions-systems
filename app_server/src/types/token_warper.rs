use axum::extract::FromRequestParts;
use axum_extra::extract::CookieJar;
use domain::entity::token::Token;

use crate::error::AuthError;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TokenWrap(pub Token);

impl<S> FromRequestParts<S> for TokenWrap
where
    S: Send + Sync,
{
    type Rejection = crate::error::AuthError;
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
            let jar = CookieJar::from_request_parts(parts, state)
                .await
                .map_err(|_| AuthError::InvalidToken)?;
            match jar.get("token") {
                Some(cookie) => {
                    let v = cookie.value();
                    let token = TokenWrap(Token(v.to_string()));
                    Ok(token)
                }
                None => Err(AuthError::InvalidToken),
            }
        })
    }
}
