use std::{future::Future, pin::Pin};

use domain::{
    entity::{user::User, user_builder::UserBuilder, user_id::UserId},
    repository::{repository_error::RepositoryError, user_repository::UserRepository},
};
use sqlx::{MySql, MySqlPool, Row, Transaction};

use super::to_repo_err;

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pool: MySqlPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn create<'a>(
        &'a self,
        tx: &'a mut Transaction<'_, MySql>,
        user_builder: &'a UserBuilder,
    ) -> Pin<Box<dyn Future<Output = Result<UserId, RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let affected_len = sqlx::query(
                r#"
                    INSERT INTO users
                    (user_id, user_name, email, password_hash)
                    VALUES
                    (?, ?, ?, ?)
                "#,
            )
            .bind(&user_builder.user_id.0)
            .bind(&user_builder.user_name)
            .bind(&user_builder.email)
            .bind(&user_builder.password_hash)
            .execute(&mut **tx)
            .await
            .map_err(|e| to_repo_err(e))?
            .rows_affected();

            if affected_len == 1 {
                Ok(user_builder.user_id.to_owned())
            } else {
                Err(RepositoryError::DatabaseError(
                    "Failed to insert".to_string(),
                ))
            }
        })
    }

    fn find_by_id<'a>(
        &'a self,
        id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<User, RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let user = sqlx::query_as::<_, User>(
                r#"
                    SELECT user_id, user_name, email, password_hash FROM users
                    WHERE user_id = ?
                "#,
            )
            .bind(&id.0)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| to_repo_err(e))?;
            Ok(user)
        })
    }

    fn find_by_email<'a>(
        &'a self,
        email: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<User, RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let user = sqlx::query_as::<_, User>(
                r#"
                    SELECT user_id, user_name, email, password_hash FROM users
                    WHERE email = ?
                "#,
            )
            .bind(&email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| to_repo_err(e))?;
            Ok(user)
        })
    }

    fn update<'a>(
        &'a self,
        user: &'a User,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let affected_len = sqlx::query(
                r#"
                    UPDATE users
                    SET
                    user_name = ?,
                    email = ?,
                    password_hash = ?
                    WHERE user_id = ?
                "#,
            )
            .bind(&user.user_name)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(&user.user_id.0)
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

    fn delete<'a>(
        &'a self,
        id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>> {
        let id = id.to_owned();
        Box::pin(async move {
            let affected_len = sqlx::query(
                r#"
                    DELETE FROM users
                    WHERE user_id = ?
                "#,
            )
            .bind(&id.0)
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

    fn is_exist<'a>(
        &'a self,
        email: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let row = sqlx::query(
                r#"
                    SELECT EXISTS (
                        SELECT 1
                        FROM users
                        WHERE email = ?
                    ) AS is_exist
                "#,
            )
            .bind(&email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| to_repo_err(e))?;

            let is_exist = row
                .try_get("is_exist")
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            Ok(is_exist)
        })
    }
}

#[cfg(test)]
mod test {
    use domain::{
        entity::{user::User, user_builder::UserBuilder, user_id::UserId},
        repository::user_repository::UserRepository,
    };
    use sqlx::MySqlPool;
    use uuid::Uuid;

    use crate::repository::user_repository_impl::UserRepositoryImpl;

    type MyResult<T> = Result<T, Box<dyn std::error::Error + 'static>>;

    async fn gen_pool() -> MyResult<MySqlPool> {
        let database_url = dotenvy::var("DATABASE_URL")?;
        let pool = MySqlPool::connect(&database_url).await?;
        Ok(pool)
    }

    #[tokio::test]
    async fn test_create() -> MyResult<()> {
        let pool = gen_pool().await?;
        let service = UserRepositoryImpl::new(pool.clone());
        let (expected_user_id, builder) = builder();
        let mut tx = pool.begin().await.unwrap();
        let returned_user_id = service.create(&mut tx, &builder).await?;
        tx.commit().await.unwrap();
        assert_eq!(returned_user_id, expected_user_id);
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_id() -> MyResult<()> {
        let (expected_user_id, builder) = builder();
        let returned_user_id = create_user_batch(expected_user_id.clone(), builder.clone()).await?;

        let service = UserRepositoryImpl::new(gen_pool().await?);
        let returned_user = service.find_by_id(&returned_user_id).await?;
        assert_filed(returned_user, builder);
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_email() -> MyResult<()> {
        let (expected_user_id, builder) = builder();
        create_user_batch(expected_user_id.clone(), builder.clone()).await?;

        let service = UserRepositoryImpl::new(gen_pool().await?);
        let returned_user = service.find_by_email(&builder.email).await?;
        assert_filed(returned_user, builder);
        Ok(())
    }

    #[tokio::test]
    async fn test_update() -> MyResult<()> {
        let (expected_user_id, mut builder) = builder();
        let user_id = create_user_batch(expected_user_id.clone(), builder.clone()).await?;

        // Update fields
        builder.user_name = format!("updated_user_name_{}", user_id.0);
        builder.email = format!("updated_user_{}", user_id.0);
        builder.password_hash = format!("updated_password_{}", user_id.0);

        let updated_user = User {
            user_id: user_id.clone(),
            user_name: builder.user_name.clone(),
            email: builder.email.clone(),
            password_hash: builder.password_hash.clone(),
        };

        let service = UserRepositoryImpl::new(gen_pool().await?);
        service.update(&updated_user).await?;

        let returned_user = service.find_by_id(&user_id).await?;
        assert_filed(returned_user, builder);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> MyResult<()> {
        let (user_id, builder) = builder();
        create_user_batch(user_id.clone(), builder.clone()).await?;

        let service = UserRepositoryImpl::new(gen_pool().await?);

        // Perform delete
        service.delete(&user_id).await?;

        // Verify user no longer exists
        let user_exists = service.is_exist(&builder.email).await?;
        assert!(!user_exists);

        Ok(())
    }

    #[tokio::test]
    async fn test_is_exist() -> MyResult<()> {
        let (user_id, builder) = builder();
        create_user_batch(user_id, builder.clone()).await?;

        let service = UserRepositoryImpl::new(gen_pool().await?);
        // Check existence of the user
        let exists = service.is_exist(&builder.email).await?;
        assert!(exists);

        // Check non-existent email
        let non_exists = service.is_exist("nonexistent_email@mail.com").await?;
        assert!(!non_exists);
        Ok(())
    }

    // Helper methods
    fn builder() -> (UserId, UserBuilder) {
        let random_string = Uuid::new_v4().to_string();

        let user_id = UserId(format!("test_user_id_{}", random_string));
        let builder = UserBuilder {
            user_id: user_id.clone(),
            user_name: format!("test_user_{}", random_string),
            email: format!("test_email@mail.com_{}", random_string),
            password_hash: format!("test_pass_{}", random_string),
        };
        (user_id, builder)
    }

    async fn create_user_batch(expected_user_id: UserId, builder: UserBuilder) -> MyResult<UserId> {
        let pool = gen_pool().await?;
        let service = UserRepositoryImpl::new(pool.clone());
        let mut tx = pool.begin().await.unwrap();
        let returned_user_id = service.create(&mut tx, &builder).await?;
        tx.commit().await.unwrap();
        assert_eq!(returned_user_id, expected_user_id);
        Ok(returned_user_id)
    }

    fn assert_filed(returned: User, expected: UserBuilder) {
        assert_eq!(returned.user_id, expected.user_id);
        assert_eq!(returned.user_name, expected.user_name);
        assert_eq!(returned.email, expected.email);
        assert_eq!(returned.password_hash, expected.password_hash);
    }
}
