use axum::{http::StatusCode, response::IntoResponse};
use domain::service::service_error::{
    auth_service_error::AuthServiceError, daily_mission_service_error::DailyMissionServiceError,
    exp_error::ExpServiceError,
};

#[derive(Debug, Clone)]
pub enum ServerError {
    AuthError(AuthServiceError),
    DailyError(DailyMissionServiceError),
    UserExp(ExpServiceError),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::AuthError(e) => match e {
                AuthServiceError::CreateToken(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
                AuthServiceError::WrongPassword => (StatusCode::UNAUTHORIZED).into_response(),
                AuthServiceError::HashError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
                AuthServiceError::RepositoryError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
                }
            },
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
        }
    }
}
