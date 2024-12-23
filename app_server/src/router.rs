use axum::{
    routing::{get, post},
    Router,
};
use sqlx::MySqlPool;

use crate::handlers::{auth, daily_mission};

pub fn app(pool: MySqlPool) -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route(
            "/daily",
            post(daily_mission::create).get(daily_mission::get_all),
        )
        .route(
            "/daily/:id",
            get(daily_mission::get_one)
                .post(daily_mission::update)
                .delete(daily_mission::delete),
        )
        .route("/daily/set/:id", get(daily_mission::set_complete))
        .with_state(pool)
}
