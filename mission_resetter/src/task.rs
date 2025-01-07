use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use chrono::{NaiveTime, Utc};
use sqlx::MySqlPool;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::sleep,
};
use tracing::{info, warn};

use crate::{
    sql::{get_all_user_id, update_tasks},
    time::{next_event_date_time, remain},
};

// 時間を取得し、任意の時間でUPDATE文を発行するように命令する
pub(crate) fn time_handler(
    tx: Sender<bool>,
    event_time: NaiveTime,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    Box::pin(async move {
        loop {
            // 現在時刻を取得
            let current_date_time = Utc::now();
            // SQL-UPDATE文が発行される、日付-時間を決める
            let next_event_date_time =
                next_event_date_time(current_date_time.naive_utc(), event_time);
            // 現在時刻とUPDATE文が発行されるまでの残り時間を測定
            let remain =
                remain(current_date_time.naive_utc(), next_event_date_time).num_milliseconds();
            // 負の値になった時はエラー
            if remain <= 0 {
                warn!("Error: Remaining time is non-positive");
                continue;
            }
            // UPDATE実行時間までスレッドスリープ
            sleep(Duration::from_millis(remain as u64)).await;
            // DBと繋いでるタスクに、UPDATE文を発行するように指令する
            if tx.send(true).await.is_err() {
                warn!("Error: Sending message failed");
            }
        }
    })
}

// DBと接続してUPDATE文を発行
// 全ユーザーデータ(user_id)を取得し、user_idのイテレータをまわしてユーザーごとに
// daily_missionテーブルのレコードをロックしてUPDATEする
pub(crate) async fn sql_handler(
    mut rx: Receiver<bool>,
    database_url: String,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    Box::pin(async move {
        while rx.recv().await.is_some() {
            // コネクションプールの作成
            let pool = MySqlPool::connect(&database_url).await.unwrap();
            let arc_pool = Arc::new(pool.clone());
            // ユーザーIDの取得
            let user_ids = match get_all_user_id(arc_pool.clone()).await {
                Ok(v) => v,
                Err(e) => {
                    warn!("{}", e);
                    continue;
                }
            };
            // ユーザーごとにUPDATE文を発行
            let tasks = update_tasks(arc_pool, user_ids).await;
            for task in tasks {
                match task.await {
                    Ok(v) => match v {
                        Ok(_) => info!("Task success"),
                        Err(e) => warn!("{}", e),
                    },
                    Err(e) => warn!("{}", e),
                }
            }
        }
    })
}
