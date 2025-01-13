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
                ErrorRes::DATA_MISMATCH.0,
                Json(Error::new(
                    ErrorRes::DATA_MISMATCH.1,
                    ErrorRes::DATA_MISMATCH.2,
                )),
            )
                .into_response(),
            Self::InvalidData => (
                ErrorRes::INVALID_DATA.0,
                Json(Error::new(
                    ErrorRes::INVALID_DATA.1,
                    ErrorRes::INVALID_DATA.2,
                )),
            )
                .into_response(),
            Self::InvalidToken => (
                ErrorRes::INVALID_TOKEN.0,
                Json(Error::new(
                    ErrorRes::INVALID_TOKEN.1,
                    ErrorRes::INVALID_TOKEN.2,
                )),
            )
                .into_response(),
            Self::WrongPassword => (
                ErrorRes::WRONG_PASSWORD.0,
                Json(Error::new(
                    ErrorRes::WRONG_PASSWORD.1,
                    ErrorRes::WRONG_PASSWORD.2,
                )),
            )
                .into_response(),
            Self::Server => (
                ErrorRes::SERVER.0,
                Json(Error::new(ErrorRes::SERVER.1, ErrorRes::SERVER.2)),
            )
                .into_response(),
            Self::TokenExpired => (
                ErrorRes::TOKEN_EXPIRED.0,
                Json(Error::new(ErrorRes::SERVER.1, ErrorRes::SERVER.2)),
            )
                .into_response(),
            Self::UserNotFound => (
                ErrorRes::USER_NOT_FOUND.0,
                Json(Error::new(
                    ErrorRes::USER_NOT_FOUND.1,
                    ErrorRes::USER_NOT_FOUND.2,
                )),
            )
                .into_response(),
        }
    }
}

pub(crate) enum ExpError {
    DataMismatch,
    InvalidData,
    InvalidToken,
    Server,
    TokenExpired,
    NotFound,
    ExpOverflow,
}

impl From<ExpServiceError> for ExpError {
    fn from(value: ExpServiceError) -> Self {
        match value {
            ExpServiceError::AuthError(e) => match e {
                TokenServiceError::TokenInvalid(_) => ExpError::InvalidToken,
                TokenServiceError::TokenExpired => ExpError::TokenExpired,
                TokenServiceError::DataMismatch(_) => ExpError::DataMismatch,
                _ => ExpError::Server,
            },
            ExpServiceError::RepositoryError(e) => match e {
                RepositoryError::NotFound => ExpError::NotFound,
                RepositoryError::InvalidData(_) => ExpError::InvalidData,
                RepositoryError::DatabaseError(_) => ExpError::Server,
            },
            ExpServiceError::DetectedExpOverflow(_) => ExpError::ExpOverflow,
        }
    }
}

impl IntoResponse for ExpError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DataMismatch => (
                ErrorRes::DATA_MISMATCH.0,
                Json(Error::new(
                    ErrorRes::DATA_MISMATCH.1,
                    ErrorRes::DATA_MISMATCH.2,
                )),
            )
                .into_response(),
            Self::InvalidData => (
                ErrorRes::INVALID_DATA.0,
                Json(Error::new(
                    ErrorRes::INVALID_DATA.1,
                    ErrorRes::INVALID_DATA.2,
                )),
            )
                .into_response(),
            Self::InvalidToken => (
                ErrorRes::INVALID_TOKEN.0,
                Json(Error::new(
                    ErrorRes::INVALID_TOKEN.1,
                    ErrorRes::INVALID_TOKEN.2,
                )),
            )
                .into_response(),
            Self::Server => (
                ErrorRes::SERVER.0,
                Json(Error::new(ErrorRes::SERVER.1, ErrorRes::SERVER.2)),
            )
                .into_response(),
            Self::TokenExpired => (
                ErrorRes::TOKEN_EXPIRED.0,
                Json(Error::new(ErrorRes::SERVER.1, ErrorRes::SERVER.2)),
            )
                .into_response(),
            Self::NotFound => (
                ErrorRes::ENTITY_NOT_FOUND.0,
                Json(Error::new(
                    ErrorRes::ENTITY_NOT_FOUND.1,
                    ErrorRes::ENTITY_NOT_FOUND.2,
                )),
            )
                .into_response(),
            Self::ExpOverflow => (
                ErrorRes::EXP_OVERFLOW.0,
                Json(Error::new(
                    ErrorRes::EXP_OVERFLOW.1,
                    ErrorRes::EXP_OVERFLOW.2,
                )),
            )
                .into_response(),
        }
    }
}

pub(crate) enum DailyError {
    DataMismatch,
    InvalidData,
    InvalidToken,
    OverCap,
    Server,
    TokenExpired,
    EntityNotFound,
    Validate(String),
}

impl From<DailyMissionServiceError> for DailyError {
    fn from(value: DailyMissionServiceError) -> Self {
        match value {
            DailyMissionServiceError::AuthError(e) => match e {
                TokenServiceError::TokenInvalid(_) => DailyError::InvalidToken,
                TokenServiceError::TokenExpired => DailyError::TokenExpired,
                TokenServiceError::DataMismatch(_) => DailyError::DataMismatch,
                _ => DailyError::Server,
            },
            DailyMissionServiceError::RepositoryError(v) => match v {
                RepositoryError::NotFound => DailyError::EntityNotFound,
                RepositoryError::InvalidData(_) => DailyError::InvalidData,
                RepositoryError::DatabaseError(_) => DailyError::Server,
            },
            DailyMissionServiceError::OverCapacity => DailyError::OverCap,
            DailyMissionServiceError::Validate(e) => DailyError::Validate(e.to_string()),
            DailyMissionServiceError::UnknownError(_) => DailyError::Server,
        }
    }
}

impl IntoResponse for DailyError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DataMismatch => (
                ErrorRes::DATA_MISMATCH.0,
                Json(Error::new(
                    ErrorRes::DATA_MISMATCH.1,
                    ErrorRes::DATA_MISMATCH.2,
                )),
            )
                .into_response(),
            Self::InvalidData => (
                ErrorRes::INVALID_DATA.0,
                Json(Error::new(
                    ErrorRes::INVALID_DATA.1,
                    ErrorRes::INVALID_DATA.2,
                )),
            )
                .into_response(),
            Self::InvalidToken => (
                ErrorRes::INVALID_TOKEN.0,
                Json(Error::new(
                    ErrorRes::INVALID_TOKEN.1,
                    ErrorRes::INVALID_TOKEN.2,
                )),
            )
                .into_response(),
            Self::Server => (
                ErrorRes::SERVER.0,
                Json(Error::new(ErrorRes::SERVER.1, ErrorRes::SERVER.2)),
            )
                .into_response(),
            Self::TokenExpired => (
                ErrorRes::TOKEN_EXPIRED.0,
                Json(Error::new(ErrorRes::SERVER.1, ErrorRes::SERVER.2)),
            )
                .into_response(),
            Self::EntityNotFound => (
                ErrorRes::ENTITY_NOT_FOUND.0,
                Json(Error::new(
                    ErrorRes::ENTITY_NOT_FOUND.1,
                    ErrorRes::ENTITY_NOT_FOUND.2,
                )),
            )
                .into_response(),
            Self::OverCap => (
                ErrorRes::DAILY_OVER_CAP.0,
                Json(Error::new(
                    ErrorRes::DAILY_OVER_CAP.1,
                    ErrorRes::DAILY_OVER_CAP.2,
                )),
            )
                .into_response(),
            Self::Validate(e) => (
                ErrorRes::VALIDATION.0,
                Json(Error::new(
                    ErrorRes::VALIDATION.1,
                    &format!("{}:{}", ErrorRes::VALIDATION.2, e),
                )),
            )
                .into_response(),
        }
    }
}

pub(crate) enum UserError {
    DataMismatch,
    InvalidToken,
    Server,
    TokenExpired,
    UserNotFound,
    UserAlreadyExists,
    Validate(String),
}

impl From<UserServiceError> for UserError {
    fn from(value: UserServiceError) -> Self {
        match value {
            UserServiceError::UserAlreadyExists => Self::UserAlreadyExists,
            UserServiceError::UserNotFound => Self::UserNotFound,
            UserServiceError::Validation(e) => Self::Validate(e.to_string()),
            UserServiceError::TokenError(e) => match e {
                TokenServiceError::TokenInvalid(_) => Self::InvalidToken,
                TokenServiceError::TokenExpired => Self::TokenExpired,
                TokenServiceError::DataMismatch(_) => Self::DataMismatch,
                _ => Self::Server,
            },
            _ => Self::Server,
        }
    }
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DataMismatch => (
                ErrorRes::DATA_MISMATCH.0,
                Json(Error::new(
                    ErrorRes::DATA_MISMATCH.1,
                    ErrorRes::DATA_MISMATCH.2,
                )),
            )
                .into_response(),
            Self::InvalidToken => (
                ErrorRes::INVALID_TOKEN.0,
                Json(Error::new(
                    ErrorRes::INVALID_TOKEN.1,
                    ErrorRes::INVALID_TOKEN.2,
                )),
            )
                .into_response(),
            Self::Server => (
                ErrorRes::SERVER.0,
                Json(Error::new(ErrorRes::SERVER.1, ErrorRes::SERVER.2)),
            )
                .into_response(),
            Self::TokenExpired => (
                ErrorRes::TOKEN_EXPIRED.0,
                Json(Error::new(ErrorRes::SERVER.1, ErrorRes::SERVER.2)),
            )
                .into_response(),
            Self::UserNotFound => (
                ErrorRes::USER_NOT_FOUND.0,
                Json(Error::new(
                    ErrorRes::USER_NOT_FOUND.1,
                    ErrorRes::USER_NOT_FOUND.2,
                )),
            )
                .into_response(),
            Self::Validate(e) => (
                ErrorRes::VALIDATION.0,
                Json(Error::new(
                    ErrorRes::VALIDATION.1,
                    &format!("{}:{}", ErrorRes::VALIDATION.2, e),
                )),
            )
                .into_response(),
            Self::UserAlreadyExists => (
                ErrorRes::USER_ALREADY_EXISTS.0,
                Json(Error::new(
                    ErrorRes::USER_ALREADY_EXISTS.1,
                    ErrorRes::USER_ALREADY_EXISTS.2,
                )),
            )
                .into_response(),
        }
    }
}

pub(crate) enum CombineError {
    Transaction,
    Server,
    InvalidToken,
    TokenExpired,
    DataMismatch,
    UserNotFound,
    InvalidData,
    ExpOverflow,
    OverCap,
    EntityNotFound,
    Validate(String),
}

impl From<ExpServiceError> for CombineError {
    fn from(value: ExpServiceError) -> Self {
        match value {
            ExpServiceError::AuthError(e) => match e {
                TokenServiceError::TokenInvalid(_) => CombineError::InvalidToken,
                TokenServiceError::TokenExpired => CombineError::TokenExpired,
                TokenServiceError::DataMismatch(_) => CombineError::DataMismatch,
                _ => CombineError::Server,
            },
            ExpServiceError::RepositoryError(e) => match e {
                RepositoryError::NotFound => CombineError::UserNotFound,
                RepositoryError::InvalidData(_) => CombineError::InvalidData,
                RepositoryError::DatabaseError(_) => CombineError::Server,
            },
            ExpServiceError::DetectedExpOverflow(_) => CombineError::ExpOverflow,
        }
    }
}

impl From<DailyMissionServiceError> for CombineError {
    fn from(value: DailyMissionServiceError) -> Self {
        match value {
            DailyMissionServiceError::AuthError(e) => match e {
                TokenServiceError::TokenInvalid(_) => CombineError::InvalidToken,
                TokenServiceError::TokenExpired => CombineError::TokenExpired,
                TokenServiceError::DataMismatch(_) => CombineError::DataMismatch,
                _ => CombineError::Server,
            },
            DailyMissionServiceError::RepositoryError(v) => match v {
                RepositoryError::NotFound => CombineError::EntityNotFound,
                RepositoryError::InvalidData(_) => CombineError::InvalidData,
                RepositoryError::DatabaseError(_) => CombineError::Server,
            },
            DailyMissionServiceError::OverCapacity => CombineError::OverCap,
            DailyMissionServiceError::Validate(e) => CombineError::Validate(e.to_string()),
            DailyMissionServiceError::UnknownError(_) => CombineError::Server,
        }
    }
}

impl IntoResponse for CombineError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DataMismatch => (
                ErrorRes::DATA_MISMATCH.0,
                Json(Error::new(
                    ErrorRes::DATA_MISMATCH.1,
                    ErrorRes::DATA_MISMATCH.2,
                )),
            )
                .into_response(),
            Self::InvalidData => (
                ErrorRes::INVALID_DATA.0,
                Json(Error::new(
                    ErrorRes::INVALID_DATA.1,
                    ErrorRes::INVALID_DATA.2,
                )),
            )
                .into_response(),
            Self::InvalidToken => (
                ErrorRes::INVALID_TOKEN.0,
                Json(Error::new(
                    ErrorRes::INVALID_TOKEN.1,
                    ErrorRes::INVALID_TOKEN.2,
                )),
            )
                .into_response(),
            Self::Server => (
                ErrorRes::SERVER.0,
                Json(Error::new(ErrorRes::SERVER.1, ErrorRes::SERVER.2)),
            )
                .into_response(),
            Self::TokenExpired => (
                ErrorRes::TOKEN_EXPIRED.0,
                Json(Error::new(ErrorRes::SERVER.1, ErrorRes::SERVER.2)),
            )
                .into_response(),
            Self::UserNotFound => (
                ErrorRes::USER_NOT_FOUND.0,
                Json(Error::new(
                    ErrorRes::USER_NOT_FOUND.1,
                    ErrorRes::USER_NOT_FOUND.2,
                )),
            )
                .into_response(),
            Self::OverCap => (
                ErrorRes::DAILY_OVER_CAP.0,
                Json(Error::new(
                    ErrorRes::DAILY_OVER_CAP.1,
                    ErrorRes::DAILY_OVER_CAP.2,
                )),
            )
                .into_response(),
            Self::Validate(e) => (
                ErrorRes::VALIDATION.0,
                Json(Error::new(
                    ErrorRes::VALIDATION.1,
                    &format!("{}:{}", ErrorRes::VALIDATION.2, e),
                )),
            )
                .into_response(),
            Self::ExpOverflow => (
                ErrorRes::EXP_OVERFLOW.0,
                Json(Error::new(
                    ErrorRes::EXP_OVERFLOW.1,
                    ErrorRes::EXP_OVERFLOW.2,
                )),
            )
                .into_response(),
            Self::EntityNotFound => (
                ErrorRes::ENTITY_NOT_FOUND.0,
                Json(Error::new(
                    ErrorRes::ENTITY_NOT_FOUND.1,
                    ErrorRes::ENTITY_NOT_FOUND.2,
                )),
            )
                .into_response(),
            Self::Transaction => (
                ErrorRes::SERVER.0,
                Json(Error::new(ErrorRes::SERVER.1, ErrorRes::SERVER.2)),
            )
                .into_response(),
        }
    }
}
struct ErrorRes;

impl ErrorRes {
    const DATA_MISMATCH: (StatusCode, u32, &str) =
        { (StatusCode::BAD_REQUEST, 100, "Data mismatch") };

    const INVALID_DATA: (StatusCode, u32, &str) =
        { (StatusCode::BAD_REQUEST, 101, "Invalid data") };

    const INVALID_TOKEN: (StatusCode, u32, &str) =
        { (StatusCode::UNAUTHORIZED, 102, "Invalid token") };

    const SERVER: (StatusCode, u32, &str) =
        { (StatusCode::INTERNAL_SERVER_ERROR, 103, "Server Error") };

    const TOKEN_EXPIRED: (StatusCode, u32, &str) =
        { (StatusCode::UNAUTHORIZED, 104, "Token expired") };

    const USER_NOT_FOUND: (StatusCode, u32, &str) =
        { (StatusCode::NOT_FOUND, 105, "User not found") };
        
    const WRONG_PASSWORD: (StatusCode, u32, &str) =
    { (StatusCode::BAD_REQUEST, 106, "Wrong password") };
        
    const VALIDATION: (StatusCode, u32, &str) =
    { (StatusCode::BAD_REQUEST, 107, "Validation error") };
        
    const ENTITY_NOT_FOUND: (StatusCode, u32, &str) = 
        { (StatusCode::NOT_FOUND, 108, "Entity not found") };
        
    const EXP_OVERFLOW: (StatusCode, u32, &str) =
        { (StatusCode::BAD_REQUEST, 200, "Exp is fulled") };

    const DAILY_OVER_CAP: (StatusCode, u32, &str) = {
        (
            StatusCode::BAD_REQUEST,
            300,
            "The number of DailyMissions is fulled",
        )
    };

    const USER_ALREADY_EXISTS: (StatusCode, u32, &str) =
        { (StatusCode::BAD_REQUEST, 400, "User already exists") };
}
