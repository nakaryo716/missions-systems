use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::{
    entity::{daily_mission_id::DailyMissionId, daily_mission_input::DailyMissionInput},
    service::daily_mission_service::DailyMissionService,
};
use infrastructure::{
    repository::daily_mission_repository_impl::DailyMissionRepositoryImpl,
    service::{token_service_impl::TokenServiceImpl, uuid_service_impl::UUIDServiceImpl},
};
use sqlx::MySqlPool;

use crate::{error::ServerError, types::token_warper::TokenWrap};

pub async fn create(
    TokenWrap(token): TokenWrap,
    State(pool): State<MySqlPool>,
    Json(mission_payload): Json<DailyMissionInput>,
) -> Result<impl IntoResponse, ServerError> {
    let service = daily_mission_service(pool);
    service
        .create(token, mission_payload)
        .await
        .map_err(|e| ServerError::DailyError(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn get_one(
    TokenWrap(token): TokenWrap,
    State(pool): State<MySqlPool>,
    Path(mission_id): Path<String>,
) -> Result<impl IntoResponse, ServerError> {
    let service = daily_mission_service(pool);
    let mission = service
        .find_by_id(token, DailyMissionId(mission_id))
        .await
        .map_err(|e| ServerError::DailyError(e))?;
    Ok((StatusCode::OK, Json(mission)))
}

pub async fn get_all(
    TokenWrap(token): TokenWrap,
    State(pool): State<MySqlPool>,
) -> Result<impl IntoResponse, ServerError> {
    let service = daily_mission_service(pool);
    let missions = service
        .find_all(token)
        .await
        .map_err(|e| ServerError::DailyError(e))?;
    Ok((StatusCode::OK, Json(missions)))
}

pub async fn update(
    TokenWrap(token): TokenWrap,
    State(pool): State<MySqlPool>,
    Path(mission_id): Path<String>,
    Json(mission_payload): Json<DailyMissionInput>,
) -> Result<impl IntoResponse, ServerError> {
    let service = daily_mission_service(pool);
    service
        .update(token, DailyMissionId(mission_id), mission_payload)
        .await
        .map_err(|e| ServerError::DailyError(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete(
    TokenWrap(token): TokenWrap,
    State(pool): State<MySqlPool>,
    Path(mission_id): Path<String>,
) -> Result<impl IntoResponse, ServerError> {
    let service = daily_mission_service(pool);
    service
        .delete(token, DailyMissionId(mission_id))
        .await
        .map_err(|e| ServerError::DailyError(e))?;
    Ok(StatusCode::NO_CONTENT)
}

pub(super)fn daily_mission_service(
    pool: MySqlPool,
) -> DailyMissionService<TokenServiceImpl, UUIDServiceImpl, DailyMissionRepositoryImpl> {
    DailyMissionService::new(
        TokenServiceImpl,
        UUIDServiceImpl,
        DailyMissionRepositoryImpl::new(pool),
    )
}
