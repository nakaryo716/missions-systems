use std::{future::Future, pin::Pin};

use domain::{
    entity::{daily_mission::DailyMission, daily_mission_id::DailyMissionId, user_id::UserId},
    repository::{
        daily_mission_repository::DailyMissionRepository, repository_error::RepositoryError,
    },
};
use sqlx::MySqlPool;

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
    fn create(
        &self,
        builder: &DailyMission,
    ) -> Pin<Box<dyn Future<Output = Result<DailyMissionId, RepositoryError>> + Send + 'static>>
    {
        let pool = self.pool.to_owned();
        let builder = builder.to_owned();
        Box::pin(async move {
            sqlx::query(
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
            .execute(&pool)
            .await
            .map_err(|e| to_repo_err(e))?;
            Ok(builder.mission_id)
        })
    }

    fn find_by_id(
        &self,
        mission_id: &DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<DailyMission, RepositoryError>> + Send + 'static>> {
        let pool = self.pool.to_owned();
        let mission_id = mission_id.to_owned();
        Box::pin(async move {
            let mission = sqlx::query_as(
                r#"
                    SELECT user_id, mission_id, title, descriptions, is_complete
                    FROM daily_mission
                    WHERE mission_id = ?
                "#,
            )
            .bind(&mission_id.0)
            .fetch_one(&pool)
            .await
            .map_err(|e| to_repo_err(e))?;
            Ok(mission)
        })
    }

    fn find_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DailyMission>, RepositoryError>> + Send + 'static>>
    {
        let pool = self.pool.to_owned();
        let user_id = user_id.to_owned();
        Box::pin(async move {
            let missions = sqlx::query_as(
                r#"
                    SELECT user_id, mission_id, title, descriptions, is_complete
                    FROM daily_mission
                    WHERE user_id = ?
                "#,
            )
            .bind(&user_id.0)
            .fetch_all(&pool)
            .await
            .map_err(|e| to_repo_err(e))?;
            Ok(missions)
        })
    }

    fn update(
        &self,
        mission: &DailyMission,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>> {
        let pool = self.pool.to_owned();
        let mission = mission.to_owned();
        Box::pin(async move {
            sqlx::query(
                r#"
                UPDATE daily_mission
                SET
                title = ?,
                descriptions = ?
                "#,
            )
            .bind(&mission.title)
            .bind(&mission.description)
            .execute(&pool)
            .await
            .map_err(|e| to_repo_err(e))?;
            Ok(())
        })
    }

    fn set_complete_true(
        &self,
        mission_id: &DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>> {
        let pool = self.pool.to_owned();
        let mission_id = mission_id.to_owned();
        Box::pin(async move {
            sqlx::query(
                r#"
                UPDATE daily_mission
                SET is_complete = true
                WHERE mission_id = ?
                "#,
            )
            .bind(&mission_id.0)
            .execute(&pool)
            .await
            .map_err(|e| to_repo_err(e))?;
            Ok(())
        })
    }

    fn delete(
        &self,
        mission_id: &DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>> {
        let pool = self.pool.to_owned();
        let mission_id = mission_id.to_owned();
        Box::pin(async move {
            sqlx::query(
                r#"
                DELETE FROM daily_mission
                WHERE mission_id = ?
                "#,
            )
            .bind(&mission_id.0)
            .execute(&pool)
            .await
            .map_err(|e| to_repo_err(e))?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use base64::{prelude::BASE64_STANDARD, Engine};
    use domain::{
        entity::{
            daily_mission::DailyMission, daily_mission_builder::DailyMissionBuilder,
            daily_mission_id::DailyMissionId, user_id::UserId,
        },
        repository::daily_mission_repository::DailyMissionRepository,
    };
    use rand_core::{OsRng, RngCore};
    use sqlx::MySqlPool;

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

        service.delete(&daily_id).await?;
        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_find_by_id_some() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let mission = gen_daily_mission(&user_id, Some("description"));
        create_daily_batch(mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        let returned_mission = service.find_by_id(&mission.mission_id).await?;
        assert_eq!(returned_mission, mission);

        service.delete(&mission.mission_id).await?;
        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_find_by_id_none() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let mission = gen_daily_mission(&user_id, None);
        create_daily_batch(mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        let returned_mission = service.find_by_id(&mission.mission_id).await?;
        assert_eq!(returned_mission, mission);

        service.delete(&mission.mission_id).await?;
        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_find_by_user_id() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let mut generated_missions = Vec::new();
        for _ in 0..10 {
            let mission = gen_daily_mission(&user_id, None);
            generated_missions.push(mission.clone());
            create_daily_batch(mission).await?;
        }

        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        let returned_mission = service.find_by_user_id(&UserId(user_id.clone())).await?;
        assert_eq!(returned_mission, generated_missions);

        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_update() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let mut mission = gen_daily_mission(&user_id, None);
        create_daily_batch(mission.clone()).await?;

        update_mission(&mut mission);

        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        service.update(&mission).await?;

        let returned_mission = service.find_by_id(&mission.mission_id).await?;
        assert_ne!(returned_mission, mission);

        service.delete(&mission.mission_id).await?;
        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_set_complete_true() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let mission = gen_daily_mission(&user_id, Some("description"));
        create_daily_batch(mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        service.set_complete_true(&mission.mission_id).await?;

        let returned_mission = service.find_by_id(&mission.mission_id).await?;

        assert_eq!(returned_mission.user_id, mission.user_id);
        assert_eq!(returned_mission.mission_id, mission.mission_id);
        assert_eq!(returned_mission.title, mission.title);
        assert_eq!(returned_mission.description, mission.description);

        assert_ne!(returned_mission.is_complete, mission.is_complete);

        service.delete(&mission.mission_id).await?;
        delete_test_user(&user_id).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_delete() -> MyResult<()> {
        let user_id = format!("{}_{}", USER_ID, gen_random_string());
        create_test_user(&user_id).await?;

        let mission = gen_daily_mission(&user_id, Some("description"));
        create_daily_batch(mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        service.delete(&mission.mission_id).await?;

        if let Ok(_) = service.find_by_id(&mission.mission_id).await {
            delete_test_user(&user_id).await?;
            panic!("daily mission must be not exist, but exist");
        }
        delete_test_user(&user_id).await?;
        Ok(())
    }

    // Helper methods
    async fn gen_pool() -> MyResult<MySqlPool> {
        let database_url = dotenvy::var("DATABASE_URL")?;
        let pool = MySqlPool::connect(&database_url).await?;
        Ok(pool)
    }

    fn gen_random_string() -> String {
        let mut key = [0, 32];
        OsRng.fill_bytes(&mut key);
        let random_string = BASE64_STANDARD.encode(key);
        random_string
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
        let mut key = [0, 32];
        OsRng.fill_bytes(&mut key);
        let random_string = BASE64_STANDARD.encode(key);

        DailyMissionBuilder::new()
            .user_id(&UserId(user_id.to_string()))
            .mission_id(&DailyMissionId(format!("mission_id_{}", random_string)))
            .title(&format!("title_{}", random_string))
            // .description(&Some(format!("description_{}", random_string)))
            .description(&description.map(|e| e.to_owned()))
            .build()
    }

    fn update_mission(daily_mission: &mut DailyMission) {
        daily_mission.title = "updated".to_string();

        if let Some(_) = daily_mission.description {
            daily_mission.description = None
        } else {
            daily_mission.description = Some("updated".to_string());
        }

        daily_mission.is_complete = !daily_mission.is_complete;
    }

    async fn create_daily_batch(mission: DailyMission) -> MyResult<()> {
        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        service.create(&mission).await?;
        Ok(())
    }
}
