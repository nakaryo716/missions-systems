use thiserror::Error;

use crate::repository::repository_error::RepositoryError;

use super::token_service_error::TokenServiceError;

#[derive(Debug, Clone, Error)]
pub enum DailyMissionServiceError {
    #[error("Authentication failed: {0}")]
    AuthError(TokenServiceError),
    #[error("Repository error: {0}")]
    RepositoryError(RepositoryError),
    #[error("Invalid input: {0}")]
    InvalidInput(TokenServiceError),
    #[error("Stored Daily Mission is full")]
    OverCapacity,
    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl From<TokenServiceError> for DailyMissionServiceError {
    fn from(value: TokenServiceError) -> Self {
        Self::AuthError(value)
    }
}

impl From<RepositoryError> for DailyMissionServiceError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
