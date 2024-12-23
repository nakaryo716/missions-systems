use router::app;
use sqlx::MySqlPool;

mod error;
mod handlers;
mod router;
mod types;

#[tokio::main]
async fn main() {
    let database_url = dotenvy::var("DATABASE_URL").expect("Failed to get database url");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to get mysql connection");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind listener");

    let app = app(pool);
    axum::serve(listener, app).await.unwrap()
}
