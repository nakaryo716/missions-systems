use std::{future::Future, pin::Pin};

use domain::{
    entity::{daily_mission::DailyMission, daily_mission_id::DailyMissionId, user_id::UserId},
    repository::{
        daily_mission_repository::DailyMissionRepository, repository_error::RepositoryError,
    },
};
use sqlx::{MySql, MySqlPool, Transaction};

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
    ) -> Pin<Box<dyn Future<Output = Result<DailyMissionId, RepositoryError>> + Send + 'a>>
    {
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
            .map_err(|e| to_repo_err(e))?
            .rows_affected();

            if affected_len == 1 {
                Ok(builder.mission_id.to_owned())
            } else {
                Err(RepositoryError::DatabaseError("Failed to insert".to_string()))
            }
        })
    }

    fn find_by_id<'a>(
        &'a self,
        mission_id: &'a DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<DailyMission, RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let mission = sqlx::query_as(
                r#"
                    SELECT user_id, mission_id, title, descriptions, is_complete
                    FROM daily_mission
                    WHERE mission_id = ?
                "#,
            )
            .bind(&mission_id.0)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| to_repo_err(e))?;
            Ok(mission)
        })
    }

    fn find_by_user_id<'a>(
        &'a self,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DailyMission>, RepositoryError>> + Send + 'a>>
    {
        Box::pin(async move {
            let missions = sqlx::query_as(
                r#"
                    SELECT user_id, mission_id, title, descriptions, is_complete
                    FROM daily_mission
                    WHERE user_id = ?
                "#,
            )
            .bind(&user_id.0)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| to_repo_err(e))?;
            Ok(missions)
        })
    }

    fn update<'a>(
        &'a self,
        mission: &'a DailyMission,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let affected_len = sqlx::query(
                r#"
                UPDATE daily_mission
                SET
                title = ?,
                descriptions = ?
                WHERE mission_id = ?
                "#,
            )
            .bind(&mission.title)
            .bind(&mission.description)
            .bind(&mission.mission_id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| to_repo_err(e))?
            .rows_affected();
            
            if affected_len == 1 {
                Ok(())
            } else {
                Err(RepositoryError::NotFound)
            }
        })
    }

    fn set_complete_true<'a>(
        &self,
        tx: &'a mut Transaction<'_, MySql>,
        mission_id: &'a DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let affected_len = sqlx::query(
                r#"
                UPDATE daily_mission
                SET is_complete = true
                WHERE mission_id = ?
                "#,
            )
            .bind(&mission_id.0)
            .execute(& mut **tx)
            .await
            .map_err(|e| to_repo_err(e))?
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
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let affected_len = sqlx::query(
                r#"
                DELETE FROM daily_mission
                WHERE mission_id = ?
                "#,
            )
            .bind(&mission_id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| to_repo_err(e))?
            .rows_affected();
            
            if affected_len == 1 {
                Ok(())
            } else {
                Err(RepositoryError::NotFound)
            }
        })
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

        service.delete(&daily_id).await?;
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

        let pool = gen_pool().await?;

        let mission = gen_daily_mission(&user_id, None);
        create_daily_batch(pool.clone(), mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(pool);
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

        let pool = gen_pool().await?;

        let mission = gen_daily_mission(&user_id, Some("description"));
        create_daily_batch(pool.clone(), mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(pool.clone());
        let mut tx = pool.begin().await?;
        service.set_complete_true(&mut tx, &mission.mission_id).await?;
        tx.commit().await?;

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

        let pool = gen_pool().await?;

        let mission = gen_daily_mission(&user_id, Some("description"));
        create_daily_batch(pool.clone(), mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(pool);
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
