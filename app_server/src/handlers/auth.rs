use std::time::Duration;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use chrono::Local;
use cookie::CookieBuilder;
use domain::{entity::auth_request::AuthRequest, service::auth_service::AuthService};
use infrastructure::{
    repository::user_repository_impl::UserRepositoryImpl,
    service::{
        password_hash_service_impl::PasswordHashServiceImpl, token_service_impl::TokenServiceImpl,
    },
};
use sqlx::MySqlPool;

use crate::{error::AuthError, COOKIE_KEY};

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
    let cookie = CookieBuilder::new(COOKIE_KEY, token.0)
        .secure(true)
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .path("/")
        .build();

    Ok((StatusCode::OK, jar.add(cookie)))
}

fn token_exp() -> usize {
    let offset_lim_time = Local::now() + Duration::new(3600, 0);
    offset_lim_time.timestamp() as usize
}
