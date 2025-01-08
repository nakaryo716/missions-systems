use axum::{http::StatusCode, response::IntoResponse, Json};
use domain::{
    repository::repository_error::RepositoryError,
    service::service_error::{
        auth_service_error::AuthServiceError,
        daily_mission_service_error::DailyMissionServiceError, exp_error::ExpServiceError,
        token_service_error::TokenServiceError, user_service_error::UserServiceError,
    },
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub enum ServerError {
    DailyError(DailyMissionServiceError),
    UserExp(ExpServiceError),
    UserErr(UserServiceError),
    Transaction(String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DailyError(e) => match e {
                DailyMissionServiceError::AuthError(_) => {
                    (StatusCode::UNAUTHORIZED).into_response()
                }
                DailyMissionServiceError::InvalidInput(_) => {
                    (StatusCode::BAD_REQUEST).into_response()
                }
                DailyMissionServiceError::RepositoryError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
                DailyMissionServiceError::OverCapacity => (StatusCode::BAD_REQUEST).into_response(),
                DailyMissionServiceError::Validate(_) => (StatusCode::BAD_REQUEST).into_response(),
                DailyMissionServiceError::UnknownError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
            },
            Self::UserExp(e) => match e {
                ExpServiceError::AuthError(_) => (StatusCode::UNAUTHORIZED).into_response(),
                ExpServiceError::RepositoryError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
                ExpServiceError::DetectedExpOverflow(_) => {
                    (StatusCode::BAD_REQUEST).into_response()
                }
            },
            Self::UserErr(e) => match e {
                UserServiceError::RepositoryError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
                UserServiceError::HashError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
                UserServiceError::TokenError(_) => (StatusCode::UNAUTHORIZED).into_response(),
                UserServiceError::UserAlreadyExists => (StatusCode::BAD_REQUEST).into_response(),
                UserServiceError::Validation(_) => (StatusCode::BAD_REQUEST).into_response(),
            },
            Self::Transaction(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Error {
    code: u32,
    message: String,
}

impl Error {
    fn new(code: u32, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum AuthError {
    DataMismatch,
    InvalidData,
    InvalidToken,
    WrongPassword,
    Server,
    TokenExpired,
    UserNotFound,
}

impl From<AuthServiceError> for AuthError {
    fn from(value: AuthServiceError) -> Self {
        match value {
            AuthServiceError::WrongPassword => AuthError::WrongPassword,
            AuthServiceError::RepositoryError(v) => match v {
                RepositoryError::NotFound => AuthError::UserNotFound,
                RepositoryError::InvalidData(_) => AuthError::InvalidData,
                RepositoryError::DatabaseError(_) => AuthError::Server,
            },
            AuthServiceError::CreateToken(e) => match e {
                TokenServiceError::TokenInvalid(_) => AuthError::InvalidToken,
                TokenServiceError::TokenExpired => AuthError::TokenExpired,
                TokenServiceError::DataMismatch(_) => AuthError::DataMismatch,
                _ => AuthError::Server,
            },
            AuthServiceError::HashError(_) => AuthError::Server,
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DataMismatch => (
                StatusCode::BAD_REQUEST,
                Json(Error::new(100, "Data mismatch")),
            )
                .into_response(),
            Self::InvalidData => (
                StatusCode::BAD_REQUEST,
                Json(Error::new(101, "Invalid data")),
            )
                .into_response(),
            Self::InvalidToken => (
                StatusCode::BAD_REQUEST,
                Json(Error::new(102, "Invalid token")),
            )
                .into_response(),
            Self::WrongPassword => (
                StatusCode::BAD_REQUEST,
                Json(Error::new(103, "Wrong password")),
            )
                .into_response(),
            Self::Server => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Error::new(104, "Server error")),
            )
                .into_response(),
            Self::TokenExpired => (
                StatusCode::BAD_REQUEST,
                Json(Error::new(105, "Token expired")),
            )
                .into_response(),
            Self::UserNotFound => (
                StatusCode::NOT_FOUND,
                Json(Error::new(106, "User not found")),
            )
                .into_response(),
        }
    }
}
