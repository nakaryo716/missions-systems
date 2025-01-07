use std::sync::Arc;

use sqlx::{Error, FromRow, MySql, MySqlPool, Transaction};
use tokio::task::JoinHandle;

#[derive(Debug, Clone, FromRow)]
pub(crate) struct UserId {
    pub user_id: String,
}

// 登録されているユーザを取得
// ユーザ数が多くなると、メモリを圧迫するためシャーディングなどを行う
// Vectorで一度保存している
pub(crate) async fn get_all_user_id(pool: Arc<MySqlPool>) -> Result<Vec<UserId>, Error> {
    let user_ids: Vec<UserId> = sqlx::query_as(r#"SELECT user_id FROM users"#)
        .fetch_all(&*pool)
        .await?;
    Ok(user_ids)
}

// daily_missionテーブルのis_completeをすべてFALSEにする
pub(crate) async fn update_tasks(
    pool: Arc<MySqlPool>,
    user_ids: Vec<UserId>,
) -> Vec<JoinHandle<Result<(), Error>>> {
    let mut tasks = Vec::new();
    for user_id in user_ids.into_iter() {
        let pool = pool.clone();
        let task: JoinHandle<Result<(), Error>> = tokio::task::spawn(async move {
            // トランザクションの開始
            let mut tx = pool.begin().await?;
            // レコードの排他ロックを行う
            lock(&user_id, &mut tx).await?;
            // UPDATEでis_complete = FALSE
            update(&user_id, &mut tx).await?;
            // コミット
            tx.commit().await?;
            Ok(())
        });
        tasks.push(task);
    }
    tasks
}

async fn lock(user_id: &UserId, transaction: &mut Transaction<'_, MySql>) -> Result<(), Error> {
    sqlx::query(
        r#"
            SELECT * FROM daily_mission
            WHERE user_id = ?
            FOR UPDATE
        "#,
    )
    .bind(&user_id.user_id)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn update(user_id: &UserId, transaction: &mut Transaction<'_, MySql>) -> Result<(), Error> {
    sqlx::query(
        r#"
            UPDATE daily_mission
            SET is_complete = FALSE
            WHERE user_id = ?
        "#,
    )
    .bind(&user_id.user_id)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
