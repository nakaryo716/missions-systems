use thiserror::Error;
use validator::ValidationErrors;

use super::{hash_error::HashServiceError, token_service_error::TokenServiceError};
use crate::repository::repository_error::RepositoryError;

#[derive(Debug, Clone, Error)]
pub enum UserServiceError {
    #[error("Hash error: {0}")]
    HashError(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Token error: {0}")]
    TokenError(TokenServiceError),
    #[error("Validation error: {0}")]
    Validation(ValidationErrors),
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
