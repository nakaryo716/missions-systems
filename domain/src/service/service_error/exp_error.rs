use thiserror::Error;

use crate::repository::repository_error::RepositoryError;

use super::token_service_error::TokenServiceError;

#[derive(Debug, Clone, Error)]
pub enum ExpServiceError {
    #[error("Authentication failed: {0}")]
    AuthError(TokenServiceError),
    #[error("Repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("Experience point is max: {0}")]
    DetectedExpOverflow(String),
}

impl From<TokenServiceError> for ExpServiceError {
    fn from(value: TokenServiceError) -> Self {
        Self::AuthError(value)
    }
}

impl From<RepositoryError> for ExpServiceError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
