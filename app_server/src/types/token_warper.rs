use axum::{body::Bytes, extract::FromRequestParts};
use domain::entity::token::Token;
use http::header::AUTHORIZATION;

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
        _state: &'life1 S,
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
            // AUTHORIZATIONヘッダから値を取得
            match parts
                .headers
                .get(AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
            {
                Some(val) => {
                    // AUTHORIZATIONヘッダの値は"Bearer <TOKEN>"の形式でBearer+空白の部分はいらない
                    // スライスをとってTOKEN部分だけを抽出する
                    let header_val_bytes = Bytes::from(val.to_owned());
                    let header_slice = header_val_bytes.slice(7..);
                    // String型に変換
                    let token = String::from_utf8(header_slice.to_vec())
                        .map_err(|_| AuthError::InvalidToken)?;
                    Ok(TokenWrap(Token(token)))
                }
                None => Err(AuthError::InvalidToken),
            }
        })
    }
}
