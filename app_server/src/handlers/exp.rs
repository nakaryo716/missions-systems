use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::service::user_exp_service::UserExpService;
use infrastructure::{
    repository::user_exp_repository_impl::UserExpRepositoryImpl,
    service::{level_convert_impl::LevelConvertImpl, token_service_impl::TokenServiceImpl},
};
use sqlx::MySqlPool;

use crate::{error::ServerError, types::token_warper::TokenWrap};

pub async fn find(
    TokenWrap(token): TokenWrap,
    State(pool): State<MySqlPool>,
) -> Result<impl IntoResponse, ServerError> {
    let service = user_exp_service(pool);
    let exp_with_level = service
        .find_with_level(token)
        .await
        .map_err(|e| ServerError::UserExp(e))?;
    Ok((StatusCode::OK, Json(exp_with_level)))
}

pub async fn add(
    TokenWrap(token): TokenWrap,
    State(pool): State<MySqlPool>,
    Path(additional_exp): Path<i32>,
) -> Result<impl IntoResponse, ServerError> {
    let service = user_exp_service(pool);
    service
        .add_experience(token, additional_exp as i64)
        .await
        .map_err(|e| ServerError::UserExp(e))?;
    Ok(())
}

pub(super) fn user_exp_service(
    pool: MySqlPool,
) -> UserExpService<UserExpRepositoryImpl, LevelConvertImpl, TokenServiceImpl> {
    UserExpService::new(
        UserExpRepositoryImpl::new(pool),
        LevelConvertImpl,
        TokenServiceImpl,
    )
}
