use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::MySqlPool;

use crate::handlers::{auth, daily_mission, exp, user};

pub fn app(pool: MySqlPool) -> Router {
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
        .route("/daily/set/:id", put(daily_mission::set_complete))
        .route("/exp", get(exp::find).put(exp::add))
        .with_state(pool)
}
