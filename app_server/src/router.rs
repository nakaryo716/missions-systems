use axum::{
    routing::{get, post, put},
    Router,
};
use http::{
    header::{
        ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS,
        ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE,
    },
    HeaderValue, Method,
};
use sqlx::MySqlPool;
use tower_http::cors::CorsLayer;

use crate::handlers::{auth, combine, daily_mission, exp, user};

pub fn app(pool: MySqlPool, allow_origin: &str) -> Router {
    Router::new()
        .route(
            "/user",
            post(user::create_and_exp_init)
                .get(user::user_info)
                .put(user::update_name)
                .delete(user::delete),
        )
        .route("/login", post(auth::login))
        .route(
            "/daily",
            post(daily_mission::create).get(daily_mission::get_all),
        )
        .route(
            "/daily/:id",
            get(daily_mission::get_one)
                .put(daily_mission::update)
                .delete(daily_mission::delete),
        )
        .route("/exp", get(exp::find))
        .route("/daily/complete/:id", put(combine::set_complete_with_add_exp))
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_origin(allow_origin.parse::<HeaderValue>().unwrap())
                .allow_credentials(true)
                .allow_methods([
                    Method::OPTIONS,
                    Method::POST,
                    Method::GET,
                    Method::PUT,
                    Method::DELETE,
                ])
                .allow_headers([
                    CONTENT_TYPE,
                    ACCESS_CONTROL_ALLOW_ORIGIN,
                    ACCESS_CONTROL_ALLOW_CREDENTIALS,
                    ACCESS_CONTROL_ALLOW_METHODS,
                ])
                .max_age(std::time::Duration::from_secs(3600)),
        )
}
