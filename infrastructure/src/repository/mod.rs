use domain::repository::repository_error::RepositoryError;
use sqlx::Error;

pub mod daily_mission_repository_impl;
pub mod user_exp_repository_impl;
pub mod user_repository_impl;

fn to_repo_err(e: sqlx::error::Error) -> RepositoryError {
    match e {
        Error::RowNotFound => RepositoryError::NotFound,
        Error::TypeNotFound { type_name } => RepositoryError::InvalidData(type_name),
        e => RepositoryError::DatabaseError(e.to_string()),
    }
}
