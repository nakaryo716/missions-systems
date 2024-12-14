use thiserror::Error;

#[derive(Debug, Error)]
pub enum HashServiceError {
    #[error("Failed to hash")]
    FailedToHash,
    #[error("Failed generate salt string")]
    FailedToGenSalt,
    #[error("Failed to verify")]
    FailedToVerify,
}
