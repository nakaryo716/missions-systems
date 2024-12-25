use chrono::NaiveTime;
use task::{sql_handler, time_handler};
use tokio::{join, sync::mpsc};
use tracing::info;

mod sql;
mod task;
mod time;

// UTC20:00:00(日本時間5:00AM)
static TARGET_TIME: NaiveTime = NaiveTime::from_hms_opt(20, 0, 0).unwrap();

#[tokio::main]
async fn main() {
    let database_url = dotenvy::var("DATABASE_URL").unwrap();
    tracing_subscriber::fmt::init();
    info!("Service Running");

    let (tx, rx) = mpsc::channel(128);
    // 時間になったら、sqlタスクを起こす
    let timer = tokio::task::spawn(time_handler(tx, TARGET_TIME));
    // UPDATE文を発行する
    let sql_task = tokio::task::spawn(sql_handler(rx, database_url).await);
    let _ = join!(timer, sql_task);
}
