use std::{future::Future, pin::Pin};

use domain::{
    entity::{daily_mission::DailyMission, daily_mission_id::DailyMissionId, user_id::UserId},
    repository::{
        daily_mission_repository::DailyMissionRepository, repository_error::RepositoryError,
    },
};
use sqlx::MySqlPool;

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
        todo!()
    }

    fn find_by_id(
        &self,
        mission_id: &DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<DailyMission, RepositoryError>> + Send + 'static>> {
        todo!()
    }

    fn find_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DailyMission>, RepositoryError>> + Send + 'static>>
    {
        todo!()
    }

    fn update(
        &self,
        mission: &DailyMission,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>> {
        todo!()
    }

    fn set_complete_true(
        &self,
        mission_id: &DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>> {
        todo!()
    }

    fn delete(
        &self,
        mission_id: &DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>> {
        todo!()
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
        repository::{
            daily_mission_repository::DailyMissionRepository, user_repository::UserRepository,
        },
    };
    use rand_core::{OsRng, RngCore};
    use sqlx::MySqlPool;

    use crate::repository::daily_mission_repository_impl::DailyMissionRepositoryImpl;

    type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

    static USER_ID: &str = "my_user_id";

    #[tokio::test]
    async fn test_daily_create() -> MyResult<()> {
        create_test_user(USER_ID).await?;

        let daily_mission = gen_daily_mission(USER_ID);
        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);

        let daily_id = service.create(&daily_mission).await?;
        assert_eq!(daily_id, daily_mission.mission_id);

        service.delete(&daily_id).await?;
        delete_test_user(USER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_find_by_id() -> MyResult<()> {
        create_test_user(USER_ID).await?;

        let mission = gen_daily_mission(USER_ID);
        create_daily_batch(mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        let returned_mission = service.find_by_id(&mission.mission_id).await?;
        assert_eq!(returned_mission, mission);

        service.delete(&mission.mission_id).await?;
        delete_test_user(USER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_find_by_user_id() -> MyResult<()> {
        create_test_user(USER_ID).await?;

        let mut generated_missions = Vec::new();
        for _ in 0..10 {
            let mission = gen_daily_mission(USER_ID);
            generated_missions.push(mission.clone());
            create_daily_batch(mission).await?;
        }

        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        let returned_mission = service
            .find_by_user_id(&UserId("user_id".to_string()))
            .await?;
        assert_eq!(returned_mission, generated_missions);

        delete_test_user(USER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_update() -> MyResult<()> {
        create_test_user(USER_ID).await?;

        let mission = gen_daily_mission(USER_ID);
        create_daily_batch(mission.clone()).await?;

        let updated_mission = update_mission(&mission);
        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        service.update(&updated_mission).await?;

        let returned_mission = service.find_by_id(&mission.mission_id).await?;
        assert_eq!(returned_mission, updated_mission);

        service.delete(&mission.mission_id).await?;
        delete_test_user(USER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_set_complete_true() -> MyResult<()> {
        create_test_user(USER_ID).await?;

        let mission = gen_daily_mission(USER_ID);
        create_daily_batch(mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        service.set_complete_true(&mission.mission_id).await?;

        let returned_mission = service.find_by_id(&mission.mission_id).await?;
        assert_ne!(returned_mission.is_complete, mission.is_complete);

        service.delete(&mission.mission_id).await?;
        delete_test_user(USER_ID).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_daily_delete() -> MyResult<()> {
        create_test_user(USER_ID).await?;

        let mission = gen_daily_mission(USER_ID);
        create_daily_batch(mission.clone()).await?;

        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        service.delete(&mission.mission_id).await?;

        if let Ok(_) = service.find_by_id(&mission.mission_id).await {
            delete_test_user(USER_ID).await?;
            panic!("daily mission must be not exist, but exist");
        }
        delete_test_user(USER_ID).await?;
        Ok(())
    }

    // Helper methods
    async fn gen_pool() -> MyResult<MySqlPool> {
        let database_url = dotenvy::var("DATABASE_URL")?;
        let pool = MySqlPool::connect(&database_url).await?;
        Ok(pool)
    }

    async fn create_test_user(user_id: &str) -> MyResult<()> {
        let pool = gen_pool().await?;
        sqlx::query(
            r#"
                INSERT INTO users
                (user_id, user_name, password_hash)
                VALUES
                (?, test_user_name, test_user_email, test_password)
            "#,
        )
        .bind(user_id)
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

    fn gen_daily_mission(user_id: &str) -> DailyMission {
        let mut key = [0, 16];
        OsRng.fill_bytes(&mut key);
        let random_string = BASE64_STANDARD.encode(key);

        DailyMissionBuilder::new()
            .user_id(&UserId(user_id.to_string()))
            .mission_id(&DailyMissionId(format!("mission_id_{}", random_string)))
            .title(&format!("title_{}", random_string))
            .description(&Some(format!("description_{}", random_string)))
            .build()
    }

    fn update_mission(daily_mission: &DailyMission) -> DailyMission {
        let mut mission = DailyMissionBuilder::new()
            .user_id(&daily_mission.user_id)
            .mission_id(&daily_mission.mission_id)
            .title("update title")
            .description(&None)
            .build();
        mission.is_complete = true;
        mission
    }

    async fn create_daily_batch(mission: DailyMission) -> MyResult<()> {
        let service = DailyMissionRepositoryImpl::new(gen_pool().await?);
        service.create(&mission).await?;
        Ok(())
    }
}
