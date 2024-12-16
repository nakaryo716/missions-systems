use thiserror::Error;

use crate::repository::repository_error::RepositoryError;

use super::{hash_error::HashServiceError, token_service_error::TokenServiceError};

#[derive(Debug, Clone, Error)]
pub enum AuthServiceError {
    #[error("Repository error")]
    RepositoryError(RepositoryError),
    #[error("Wrong password")]
    WrongPassword,
    #[error("Failed to create token")]
    CreateToken(TokenServiceError),
    #[error("Hash error")]
    HashError(HashServiceError),
}

impl From<RepositoryError> for AuthServiceError {
    fn from(value: RepositoryError) -> Self {
        AuthServiceError::RepositoryError(value)
    }
}

impl From<TokenServiceError> for AuthServiceError {
    fn from(value: TokenServiceError) -> Self {
        Self::CreateToken(value)
    }
}

impl From<HashServiceError> for AuthServiceError {
    fn from(value: HashServiceError) -> Self {
        AuthServiceError::HashError(value)
    }
}
