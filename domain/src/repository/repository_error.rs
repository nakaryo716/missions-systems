use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum RepositoryError {
    #[error("Row not found")]
    NotFound,
    #[error("Failed to connecting database")]
    DatabaseError(String),
    #[error("Invalid data")]
    InvalidData(String),
}
