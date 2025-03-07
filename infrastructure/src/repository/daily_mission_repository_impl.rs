use std::{future::Future, pin::Pin};

use domain::{
    entity::{daily_mission::DailyMission, daily_mission_id::DailyMissionId, user_id::UserId},
    repository::{
        daily_mission_repository::DailyMissionRepository, repository_error::RepositoryError,
    },
};
use sqlx::{mysql::MySqlRow, prelude::FromRow, types::chrono::{FixedOffset, NaiveDate, Utc}, MySql, MySqlPool, Row, Transaction};

use super::to_repo_err;

#[derive(Debug, Clone)]
pub struct DailyMissionRepositoryImpl {
    pool: MySqlPool,
}

impl DailyMissionRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl DailyMissionRepository for DailyMissionRepositoryImpl {
    fn create<'a>(
        &'a self,
        builder: &'a DailyMission,
    ) -> Pin<Box<dyn Future<Output = Result<DailyMissionId, RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let affected_len = sqlx::query(
                r#"
                INSERT INTO daily_mission
                (user_id, mission_id, title, descriptions)
                VALUES
                (?, ?, ?, ?)
                "#,
            )
            .bind(&builder.user_id.0)
            .bind(&builder.mission_id.0)
            .bind(&builder.title)
            .bind(&builder.description)
            .execute(&self.pool)
            .await
            .map_err(to_repo_err)?
            .rows_affected();

            if affected_len == 1 {
                Ok(builder.mission_id.to_owned())
            } else {
                Err(RepositoryError::DatabaseError(
                    "Failed to insert".to_string(),
                ))
            }
        })
    }

    fn count<'a>(
        &'a self,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<i32, RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let row = sqlx::query(
                r#"
                    SELECT COUNT(*) AS len
                    FROM daily_mission
                    WHERE user_id = ?
                "#,
            )
            .bind(&user_id.0)
            .fetch_one(&self.pool)
            .await
            .map_err(to_repo_err)?;

            let len = row.try_get("len").map_err(to_repo_err)?;
            Ok(len)
        })
    }

    // 今日のミッションを取得する
    fn find_by_id<'a>(
        &'a self,
        mission_id: &'a DailyMissionId,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<DailyMission, RepositoryError>> + Send + 'a>> {
        // 今日の日付を取得する(日本時間)
        let current_date = current_date_jp();
        Box::pin(async move {
            // daily_mission tableとmission_completed tableをJOINして
            // DailyMissionRow型としてDBから取得し、DailyMissionに変換する
            let mission: DailyMissionRow = sqlx::query_as(
                r#"
                SELECT
                daily_mission.user_id,
                daily_mission.mission_id,
                daily_mission.title, 
                daily_mission.descriptions AS description,
                mission_completed.date
                FROM daily_mission
                LEFT JOIN mission_completed
                ON daily_mission.mission_id = mission_completed.mission_id
                AND mission_completed.date = ?
                WHERE daily_mission.mission_id = ?
                AND
                daily_mission.user_id = ?
                "#,
            )
            .bind(&current_date)
            .bind(&mission_id.0)
            .bind(&user_id.0)
            .fetch_one(&self.pool)
            .await
            .map_err(to_repo_err)?;
            Ok(mission.into())
        })
    }

    // 今日のミッションすべてを取得する
    fn find_by_user_id<'a>(
        &'a self,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DailyMission>, RepositoryError>> + Send + 'a>> {
        let current_date = current_date_jp();
        // daily_mission tableとmission_completed tableをJOINして
        // DailyMissionRowをDBから取得しDailyMissionに変換する
        Box::pin(async move {
            let missions: Vec<DailyMissionRow> = sqlx::query_as(
                r#"
                    SELECT
                    daily_mission.user_id,
                    daily_mission.mission_id,
                    daily_mission.title, 
                    daily_mission.descriptions AS description,
                    mission_completed.date
                    FROM daily_mission
                    LEFT JOIN mission_completed
                    ON daily_mission.mission_id = mission_completed.mission_id
                    AND mission_completed.date = ?
                    WHERE daily_mission.user_id = ?
                "#,
            )
            .bind(&current_date)
            .bind(&user_id.0)
            .fetch_all(&self.pool)
            .await
            .map_err(to_repo_err)?;
            Ok(missions.into_iter().map(|f| f.into()).collect())
        })
    }

    fn update<'a>(
        &'a self,
        mission: &'a DailyMission,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let affected_len = sqlx::query(
                r#"
                UPDATE daily_mission
                SET
                title = ?,
                descriptions = ?
                WHERE mission_id = ? && user_id = ?
                "#,
            )
            .bind(&mission.title)
            .bind(&mission.description)
            .bind(&mission.mission_id.0)
            .bind(&user_id.0)
            .execute(&self.pool)
            .await
            .map_err(to_repo_err)?
            .rows_affected();

            if affected_len == 1 {
                Ok(())
            } else {
                Err(RepositoryError::NotFound)
            }
        })
    }

    // 今日のミッションを完了にする
    fn set_complete_true<'a>(
        &self,
        tx: &'a mut Transaction<'_, MySql>,
        mission_id: &'a DailyMissionId,
        _user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>> {
        // 今日の日付を取得する(日本時間)
        let current_date = current_date_jp();
        Box::pin(async move {
            let affected_len = sqlx::query(
                r#"
                INSERT INTO mission_completed
                (mission_id, date)
                VALUES
                (?, ?)
                "#,
            )
            .bind(&mission_id.0)
            .bind(&current_date)
            .execute(&mut **tx)
            .await
            .map_err(to_repo_err)?
            .rows_affected();

            if affected_len == 1 {
                Ok(())
            } else {
                Err(RepositoryError::NotFound)
            }
        })
    }

    fn delete<'a>(
        &'a self,
        mission_id: &'a DailyMissionId,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let affected_len = sqlx::query(
                r#"
                DELETE FROM daily_mission
                WHERE mission_id = ? && user_id = ?
                "#,
            )
            .bind(&mission_id.0)
            .bind(&user_id.0)
            .execute(&self.pool)
            .await
            .map_err(to_repo_err)?
            .rows_affected();

            if affected_len == 1 {
                Ok(())
            } else {
                Err(RepositoryError::NotFound)
            }
        })
    }
}

static JP_OFFSET: i32 = 9 * 3600;
fn current_date_jp() -> NaiveDate {
    let jp_tz = FixedOffset::east_opt(JP_OFFSET).unwrap();
    Utc::now().with_timezone(&jp_tz).date_naive()
}

/// DBのテーブルをJOINしたDBからのrowデータ
/// daily_missionテーブルとmission_completedをLEFT JOINしている
/// DailyMissionはDailyMissionRowによって生成できる
#[derive(Debug, Clone)]
struct DailyMissionRow {
    user_id: UserId,
    mission_id: DailyMissionId,
    title: String,
    description: Option<String>,
    have_complete: Option<NaiveDate>,
}

impl<'r> FromRow<'r, MySqlRow> for DailyMissionRow {
    fn from_row(row: &'r MySqlRow) -> Result<Self, sqlx::Error> {
       Ok(
        DailyMissionRow {
            user_id: UserId(row.try_get("user_id")?),
            mission_id: DailyMissionId(row.try_get("mission_id")?),
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            have_complete: row.try_get("date")?,
        }
       ) 
    }
}

impl From<DailyMissionRow> for DailyMission {
    fn from(value: DailyMissionRow) -> Self {
        DailyMission {
            user_id: value.user_id,
            mission_id: value.mission_id,
            title: value.title,
            description: value.description,
            is_complete: value.have_complete.is_some(),  
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use domain::{
        entity::{
            daily_mission::DailyMission, daily_mission_builder::DailyMissionBuilder,
            daily_mission_id::DailyMissionId, user_id::UserId,
        },
        repository::daily_mission_repository::DailyMissionRepository,
    };
    use sqlx::MySqlPool;
    use uuid::Uuid;

    use crate::repository::daily_mission_repository_impl::DailyMissionRepositoryImpl;

    type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

    static USER_ID: &str = "my_user_id";

    #[tokio::test]
    async fn test_daily_create() -> MyResult<()> {
        // gen user id
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        // create user
        create_test_user(&user_id).await?;

        let daily_mission = gen_daily_mission(&user_id, Some("hi"));
        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);

        let daily_id = service.create(&daily_mission).await?;
        assert_eq!(daily_id, daily_mission.mission_id);

        service.delete(&daily_id, &UserId(user_id.clone())).await?;
        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_count() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        for _ in 0..10 {
            let daily_mission = gen_daily_mission(&user_id, Some("hi"));
            service.create(&daily_mission).await?;
        }
        
        let count = service.count(&UserId(user_id.clone())).await?;
        assert_eq!(count, 10);

        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_find_by_id_some() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let pool = gen_pool().await?;

        let mission = gen_daily_mission(&user_id, Some("description"));
        create_daily_batch(pool.clone(), mission.clone()).await?;
        let service = DailyMissionRepositoryImpl::new(pool);

        let returned_mission = service
            .find_by_id(&mission.mission_id, &UserId(user_id.clone()))
            .await?;
        assert_eq!(returned_mission, mission);

        service
            .delete(&mission.mission_id, &UserId(user_id.clone()))
            .await?;
        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_find_by_id_none() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let pool = gen_pool().await?;

        let mission = gen_daily_mission(&user_id, None);
        create_daily_batch(pool.clone(), mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(pool);
        let returned_mission = service
            .find_by_id(&mission.mission_id, &UserId(user_id.clone()))
            .await?;
        assert_eq!(returned_mission, mission);

        service
            .delete(&mission.mission_id, &UserId(user_id.clone()))
            .await?;
        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_find_by_user_id() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let pool = gen_pool().await?;

        let mut generated_missions = HashSet::new();
        for _ in 0..5 {
            let mission = gen_daily_mission(&user_id, None);
            generated_missions.insert(mission.clone());
            create_daily_batch(pool.clone(), mission).await?;
        }

        let service = DailyMissionRepositoryImpl::new(pool);
        let returned_mission = service.find_by_user_id(&UserId(user_id.clone())).await?;

        let mut missions = HashSet::new();
        returned_mission.iter().for_each(|a| {
            missions.insert(a.to_owned());
        });
        assert_eq!(missions.len(), generated_missions.len());
        assert_eq!(missions, generated_missions);

        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_update() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let pool = gen_pool().await?;

        let mut mission = gen_daily_mission(&user_id, None);
        create_daily_batch(pool.clone(), mission.clone()).await?;

        helper_update_mission(&mut mission);

        let service = DailyMissionRepositoryImpl::new(pool);
        service.update(&mission, &UserId(user_id.clone())).await?;

        let returned_mission = service
            .find_by_id(&mission.mission_id, &UserId(user_id.clone()))
            .await?;
        assert_ne!(returned_mission, mission);

        service
            .delete(&mission.mission_id, &UserId(user_id.clone()))
            .await?;
        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_set_complete_true() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let pool = gen_pool().await?;

        let mission = gen_daily_mission(&user_id, Some("description"));
        create_daily_batch(pool.clone(), mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(pool.clone());
        let mut tx = pool.begin().await?;
        service
            .set_complete_true(&mut tx, &mission.mission_id, &UserId(user_id.clone()))
            .await?;
        tx.commit().await?;

        let returned_mission = service
            .find_by_id(&mission.mission_id, &UserId(user_id.clone()))
            .await?;

        assert_eq!(returned_mission.user_id, mission.user_id);
        assert_eq!(returned_mission.mission_id, mission.mission_id);
        assert_eq!(returned_mission.title, mission.title);
        assert_eq!(returned_mission.description, mission.description);

        assert_ne!(returned_mission.is_complete, mission.is_complete);

        service
            .delete(&mission.mission_id, &UserId(user_id.clone()))
            .await?;
        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_delete() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let pool = gen_pool().await?;

        let mission = gen_daily_mission(&user_id, Some("description"));
        create_daily_batch(pool.clone(), mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(pool);
        service
            .delete(&mission.mission_id, &UserId(user_id.clone()))
            .await?;

        if let Ok(_) = service
            .find_by_id(&mission.mission_id, &UserId(user_id.clone()))
            .await
        {
            delete_test_user(&user_id).await?;
            panic!("daily mission must be not exist, but exist");
        }
        delete_test_user(&user_id).await?;
        Ok(())
    }

    // Helper methods
    async fn gen_pool() -> MyResult<MySqlPool> {
        let database_url = dotenvy::var("TEST_DB_URL")?;
        let pool = MySqlPool::connect(&database_url).await?;
        Ok(pool)
    }

    fn gen_random_string() -> String {
        Uuid::new_v4().to_string()
    }

    async fn create_test_user(user_id: &str) -> MyResult<()> {
        let pool = gen_pool().await?;
        sqlx::query(
            r#"
                INSERT INTO users
                (user_id, user_name, email, password_hash)
                VALUES
                (?, ?, ?, ?)
            "#,
        )
        .bind(user_id)
        .bind("test_user_name")
        .bind(format!("test_user_email_{}", user_id))
        .bind("test_password")
        .execute(&pool)
        .await?;
        Ok(())
    }

    async fn delete_test_user(user_id: &str) -> MyResult<()> {
        let pool = gen_pool().await?;
        sqlx::query(
            r#"
                DELETE FROM users WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .execute(&pool)
        .await?;
        Ok(())
    }

    fn gen_daily_mission(user_id: &str, description: Option<&str>) -> DailyMission {
        let random_string = Uuid::new_v4().to_string();

        DailyMissionBuilder::new()
            .user_id(&UserId(user_id.to_string()))
            .mission_id(&DailyMissionId(format!("{}", random_string)))
            .title(&format!("title_{}", random_string))
            .description(&description.map(|e| e.to_owned()))
            .build()
    }

    fn helper_update_mission(daily_mission: &mut DailyMission) {
        daily_mission.title = "updated".to_string();

        if let Some(_) = daily_mission.description {
            daily_mission.description = None
        } else {
            daily_mission.description = Some("updated".to_string());
        }

        daily_mission.is_complete = !daily_mission.is_complete;
    }

    async fn create_daily_batch(pool: MySqlPool, mission: DailyMission) -> MyResult<()> {
        let service = DailyMissionRepositoryImpl::new(pool);
        service.create(&mission).await?;
        Ok(())
    }
}
