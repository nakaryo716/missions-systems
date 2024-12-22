use std::{future::Future, pin::Pin};

use domain::{
    entity::{user_exp::UserExp, user_id::UserId},
    repository::{repository_error::RepositoryError, user_exp_repository::UserExpRepository},
};
use sqlx::MySqlPool;

use crate::repository::to_repo_err;

#[derive(Debug, Clone)]
pub struct UserExpRepositoryImpl {
    pool: MySqlPool,
}

impl UserExpRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl UserExpRepository for UserExpRepositoryImpl {
    fn init_exp(
        &self,
        user_id: &UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>> {
        let pool = self.pool.to_owned();
        let user_id = user_id.to_owned();
        Box::pin(async move {
            let affected_len = sqlx::query(
                r#"
                    INSERT INTO user_exp
                    (user_id) VALUES (?)
                "#,
            )
            .bind(&user_id.0)
            .execute(&pool)
            .await
            .map_err(|e| to_repo_err(e))?
            .rows_affected();

            if affected_len == 1 {
                Ok(())
            } else {
                Err(RepositoryError::DatabaseError("Failed to insert".to_string()))
            }
        })
    }

    fn find_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Pin<Box<dyn Future<Output = Result<UserExp, RepositoryError>> + Send + 'static>> {
        let pool = self.pool.to_owned();
        let user_id = user_id.to_owned();
        Box::pin(async move {
            let exp = sqlx::query_as(
                r#"
                    SELECT user_id, experience_points
                    FROM user_exp
                    WHERE user_id = ?
                "#,
            )
            .bind(&user_id.0)
            .fetch_one(&pool)
            .await
            .map_err(|e| to_repo_err(e))?;
            Ok(exp)
        })
    }

    fn add_exp(
        &self,
        user_id: &UserId,
        additional_exp: i64,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>> {
        let pool = self.pool.to_owned();
        let user_id = user_id.to_owned();
        let additional_exp = additional_exp.to_owned();
        Box::pin(async move {
            let result = sqlx::query(
                r#"
                    UPDATE user_exp
                    SET experience_points = experience_points + ?
                    WHERE user_id = ?
                "#,
            )
            .bind(&additional_exp)
            .bind(&user_id.0)
            .execute(&pool)
            .await
            .map_err(|e| to_repo_err(e))?;

            if result.rows_affected() == 1 {
                Ok(())
            } else {
                Err(RepositoryError::NotFound)
            }
        })
    }
}

#[cfg(test)]
mod test {
    use domain::{
        entity::{user_exp::UserExp, user_id::UserId},
        repository::user_exp_repository::UserExpRepository,
    };
    use sqlx::MySqlPool;
    use uuid::Uuid;

    use crate::repository::user_exp_repository_impl::UserExpRepositoryImpl;

    type MyResult<T> = Result<T, Box<dyn std::error::Error>>;
    #[tokio::test]
    async fn test_user_exp_init() -> MyResult<()> {
        let pool = gen_pool().await?;
        let user_id_str = gen_random_str();

        // create user
        create_user(pool.clone(), &user_id_str).await?;
        // initialize
        UserExpRepositoryImpl::new(pool.clone())
            .init_exp(&UserId(user_id_str.clone()))
            .await?;
        // select
        let exp = select_exp(pool.clone(), &user_id_str).await?;
        // assert
        assert_eq!(exp.user_id.0, user_id_str);
        assert_eq!(exp.experience_points, 0);
        // delete
        delete_test_user(&user_id_str).await?;
        Ok(())
    }

    // ユーザーが存在しなければインサート出来ないになっているかのテスト
    #[tokio::test]
    async fn test_user_exp_init_failed() -> MyResult<()> {
        let pool = gen_pool().await?;
        let user_id_str = gen_random_str();

        let res = UserExpRepositoryImpl::new(pool.clone())
            .init_exp(&UserId(user_id_str.clone()))
            .await;

        if let Ok(_) = res {
            panic!("init exp must be error due to no users, but initialized");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_user_exp_find() -> MyResult<()> {
        let pool = gen_pool().await?;
        let user_id_str = gen_random_str();
        // create user
        create_user(pool.clone(), &user_id_str).await?;
        // init repo service
        let repo = UserExpRepositoryImpl::new(pool.clone());
        // init
        repo.init_exp(&UserId(user_id_str.clone())).await?;
        let expected_exp = select_exp(pool.clone(), &user_id_str).await?;
        // find
        let returned_exp = repo.find_by_user_id(&UserId(user_id_str.clone())).await?;
        // assert
        assert_eq!(returned_exp, expected_exp);
        // delete
        delete_test_user(&user_id_str).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_user_exp_add() -> MyResult<()> {
        let additional_exp: i64 = 100;

        let pool = gen_pool().await?;
        let user_id_str = gen_random_str();
        // create user
        create_user(pool.clone(), &user_id_str).await?;
        // init repo service
        let repo = UserExpRepositoryImpl::new(pool.clone());
        // init
        repo.init_exp(&UserId(user_id_str.clone())).await?;
        // find
        let init_exp = repo.find_by_user_id(&UserId(user_id_str.clone())).await?;
        // add
        repo.add_exp(&UserId(user_id_str.clone()), additional_exp)
            .await?;
        //find
        let added_exp = repo.find_by_user_id(&UserId(user_id_str.clone())).await?;
        //assert
        assert_eq!(added_exp.user_id, init_exp.user_id);
        assert_eq!(
            added_exp.experience_points,
            init_exp.experience_points + additional_exp
        );
        //delete
        Ok(())
    }

    async fn gen_pool() -> MyResult<MySqlPool> {
        let database_url = dotenvy::var("DATABASE_URL")?;
        let pool = MySqlPool::connect(&database_url).await?;
        Ok(pool)
    }

    fn gen_random_str() -> String {
        Uuid::new_v4().to_string()
    }

    async fn create_user(pool: MySqlPool, user_id: &str) -> MyResult<()> {
        sqlx::query(
            r#"
                INSERT INTO users
                (user_id, user_name, email, password_hash)
                VALUES
                (?, ?, ?, ?)
            "#,
        )
        .bind(format!("{}", user_id))
        .bind(format!("test_name_{}", user_id))
        .bind(format!("test_email_{}", user_id))
        .bind(format!("test_pwd_{}", user_id))
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
        .bind(format!("{}", user_id))
        .execute(&pool)
        .await?;
        Ok(())
    }

    async fn select_exp(pool: MySqlPool, user_id: &str) -> MyResult<UserExp> {
        let exp = sqlx::query_as(
            r#"
                SELECT user_id, experience_points
                FROM user_exp
                WHERE user_id = ?
            "#,
        )
        .bind(format!("{}", user_id))
        .fetch_one(&pool)
        .await?;
        Ok(exp)
    }
}
