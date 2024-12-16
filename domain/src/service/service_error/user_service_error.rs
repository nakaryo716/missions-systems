use thiserror::Error;

use super::{hash_error::HashServiceError, token_service_error::TokenServiceError};
use crate::repository::repository_error::RepositoryError;

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("Hash error: {0}")]
    HashError(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("token error: {0}")]
    TokenError(TokenServiceError),
}

impl From<HashServiceError> for UserServiceError {
    fn from(value: HashServiceError) -> Self {
        UserServiceError::HashError(value.to_string())
    }
}

impl From<RepositoryError> for UserServiceError {
    fn from(value: RepositoryError) -> Self {
        UserServiceError::RepositoryError(value.to_string())
    }
}

impl From<TokenServiceError> for UserServiceError {
    fn from(value: TokenServiceError) -> Self {
        UserServiceError::TokenError(value)
    }
}
