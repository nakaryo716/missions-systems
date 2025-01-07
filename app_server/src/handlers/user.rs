use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::{entity::user_input::UserInput, service::user_service::UserService};
use infrastructure::{
    repository::user_repository_impl::UserRepositoryImpl,
    service::{
        password_hash_service_impl::PasswordHashServiceImpl, token_service_impl::TokenServiceImpl,
        uuid_service_impl::UUIDServiceImpl,
    },
};
use sqlx::MySqlPool;

use crate::{
    error::ServerError,
    types::{token_warper::TokenWrap, update_user::UpdateUser},
};

use super::exp::user_exp_service;

pub async fn create_and_exp_init(
    State(pool): State<MySqlPool>,
    Json(user_input): Json<UserInput>,
) -> Result<impl IntoResponse, ServerError> {
    let user_service = user_service(pool.clone());
    let exp_service = user_exp_service(pool.clone());
    // トランザクション開始
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| ServerError::Transaction(e.to_string()))?;
    // 1. ユーザー作成
    let user_id = user_service
        .create_user(&mut tx, user_input)
        .await
        .map_err(ServerError::UserErr)?;
    // 2. ユーザー経験値テーブルの初期化
    exp_service
        .init_exp(&mut tx, user_id)
        .await
        .map_err(ServerError::UserExp)?;
    // コミット
    tx.commit()
        .await
        .map_err(|e| ServerError::Transaction(e.to_string()))?;
    Ok(())
}

pub async fn user_info(
    TokenWrap(token): TokenWrap,
    State(pool): State<MySqlPool>,
) -> Result<impl IntoResponse, ServerError> {
    let service = user_service(pool);
    let user_info = service
        .get_user_info(token)
        .await
        .map_err(ServerError::UserErr)?;
    Ok((StatusCode::OK, Json(user_info)))
}

pub async fn update_name(
    TokenWrap(token): TokenWrap,
    State(pool): State<MySqlPool>,
    Query(UpdateUser { user_name }): Query<UpdateUser>,
) -> Result<impl IntoResponse, ServerError> {
    let service = user_service(pool);
    service
        .update_user_name(token, user_name)
        .await
        .map_err(ServerError::UserErr)?;
    Ok(())
}

pub async fn delete(
    TokenWrap(token): TokenWrap,
    State(pool): State<MySqlPool>,
) -> Result<impl IntoResponse, ServerError> {
    let service = user_service(pool);
    service
        .delete_user(token)
        .await
        .map_err(ServerError::UserErr)?;
    Ok(())
}

fn user_service(
    pool: MySqlPool,
) -> UserService<PasswordHashServiceImpl, TokenServiceImpl, UserRepositoryImpl, UUIDServiceImpl> {
    UserService::new(
        PasswordHashServiceImpl,
        TokenServiceImpl,
        UserRepositoryImpl::new(pool),
        UUIDServiceImpl,
    )
}
