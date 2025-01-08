use std::time::Duration;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use chrono::Local;
use cookie::{CookieBuilder, SameSite};
use domain::{entity::auth_request::AuthRequest, service::auth_service::AuthService};
use infrastructure::{
    repository::user_repository_impl::UserRepositoryImpl,
    service::{
        password_hash_service_impl::PasswordHashServiceImpl, token_service_impl::TokenServiceImpl,
    },
};
use sqlx::MySqlPool;

use crate::error::AuthError;

pub async fn login(
    jar: CookieJar,
    State(pool): State<MySqlPool>,
    Json(auth_payload): Json<AuthRequest>,
) -> Result<impl IntoResponse, AuthError> {
    // ログインサービスのインスタンス化
    let service = AuthService::new(
        PasswordHashServiceImpl,
        TokenServiceImpl,
        UserRepositoryImpl::new(pool),
        token_exp(),
    );

    let token = service.login(auth_payload).await?;

    let cookie = CookieBuilder::new("token", token.0)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .build();
    Ok((jar.add(cookie), StatusCode::OK))
}

fn token_exp() -> usize {
    let offset_lim_time = Local::now() + Duration::new(3600, 0);
    offset_lim_time.timestamp() as usize
}
