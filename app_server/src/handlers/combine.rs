use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use domain::entity::daily_mission_id::DailyMissionId;
use http::StatusCode;
use sqlx::MySqlPool;

use crate::{error::CombineError, types::token_warper::TokenWrap};

use super::{daily_mission::daily_mission_service, exp::user_exp_service};

static ADDITIONAL_POINT: i64 = 2;

pub(crate) async fn set_complete_with_add_exp(
    TokenWrap(token): TokenWrap,
    State(pool): State<MySqlPool>,
    Path(mission_id): Path<String>,
) -> Result<impl IntoResponse, CombineError> {
    let daily_service = daily_mission_service(pool.clone());
    let exp_service = user_exp_service(pool.clone());

    // トランザクション開始
    let mut transaction = pool.begin().await.map_err(|_| CombineError::Server)?;
    // 1.デイリーミッションのis_completeをTRUEに変更
    daily_service
        .set_complete_true(&mut transaction, token.clone(), DailyMissionId(mission_id))
        .await?;
    // ユーザーの経験値を上昇させる
    exp_service
        .add_experience(&mut transaction, token, ADDITIONAL_POINT)
        .await?;
    // コミット
    transaction
        .commit()
        .await
        .map_err(|_| CombineError::Transaction)?;
    Ok(StatusCode::OK)
}
