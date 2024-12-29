use std::{future::Future, pin::Pin};

use domain::{
    entity::{user_exp::UserExp, user_id::UserId},
    repository::{repository_error::RepositoryError, user_exp_repository::UserExpRepository},
};
use sqlx::{MySql, MySqlPool, Transaction};

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
    fn init_exp<'a>(
        &'a self,
        tx: &'a mut Transaction<'_, MySql>,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let affected_len = sqlx::query(
                r#"
                    INSERT INTO user_exp
                    (user_id) VALUES (?)
                "#,
            )
            .bind(&user_id.0)
            .execute(&mut **tx)
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

    fn find_by_user_id<'a>(
        &'a self,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<UserExp, RepositoryError>> + Send + 'a>> {
        Box::pin(async move {
            let exp = sqlx::query_as(
                r#"
                    SELECT user_id, experience_points
                    FROM user_exp
                    WHERE user_id = ?
                "#,
            )
            .bind(&user_id.0)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| to_repo_err(e))?;
            Ok(exp)
        })
    }

    fn add_exp<'a>(
        &'a self,
        tx: &'a mut Transaction<'_, MySql>,
        user_id: &'a UserId,
        additional_exp: i64,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>> {
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
            .execute(&mut **tx)
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
        entity::{user_builder::UserBuilder, user_exp::UserExp, user_id::UserId},
        repository::{user_exp_repository::UserExpRepository, user_repository::UserRepository},
    };
    use sqlx::MySqlPool;
    use uuid::Uuid;

    use crate::repository::{user_exp_repository_impl::UserExpRepositoryImpl, user_repository_impl::UserRepositoryImpl};

    type MyResult<T> = Result<T, Box<dyn std::error::Error>>;
    #[tokio::test]
    async fn test_user_exp_init() -> MyResult<()> {
        let pool = gen_pool().await?;
        let user_id_str = gen_random_str();

        // create user
        create_user(pool.clone(), &user_id_str).await?;
        let mut tx = pool.begin().await?;
        // initialize
        UserExpRepositoryImpl::new(pool.clone())
            .init_exp(&mut tx, &UserId(user_id_str.clone()))
            .await?;
        tx.commit().await?;
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

        let mut tx = pool.begin().await?;
        let res = UserExpRepositoryImpl::new(pool.clone())
            .init_exp(&mut tx, &UserId(user_id_str.clone()))
            .await;
        tx.commit().await?;
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
        let mut tx = pool.begin().await?;
        repo.init_exp(&mut tx, &UserId(user_id_str.clone())).await?;
        tx.commit().await?;
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
        let mut tx = pool.begin().await?;
        repo.init_exp(&mut tx, &UserId(user_id_str.clone())).await?;
        tx.commit().await?;

        // find
        let init_exp = repo.find_by_user_id(&UserId(user_id_str.clone())).await?;
        // add
        let mut tx = pool.begin().await?;
        repo.add_exp(&mut tx, &UserId(user_id_str.clone()), additional_exp)
            .await?;
        tx.commit().await?;
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

    #[tokio::test]
    async fn test_transaction_create_user_exp() -> MyResult<()> {
        let pool = gen_pool().await?;
        let user_id_str = gen_random_str();

        let user = UserBuilder::new()
            .user_id(UserId(user_id_str.clone()))
            .user_name(format!("test_user_{}", user_id_str))
            .email(format!("test_email_{}", user_id_str))
            .password_hash(format!("test_user_{}", user_id_str));

        let mut tx = pool.begin().await?;

        let user_id = UserRepositoryImpl::new(pool.clone())
            .create(&mut tx, &user).await?;
        
        let _ = UserExpRepositoryImpl::new(pool.clone())
            .init_exp(&mut tx, &UserId(user_id_str.clone()))
            .await;

        tx.commit().await?;

        let user_exp = UserExpRepositoryImpl::new(pool)
            .find_by_user_id(&UserId(user_id_str.clone()))
            .await?;
        
        assert_eq!(user_exp.user_id, user_id);
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
