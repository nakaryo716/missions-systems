use std::{future::Future, pin::Pin};

use sqlx::{MySql, Transaction};

use crate::entity::{user::User, user_builder::UserBuilder, user_id::UserId};

use super::repository_error::RepositoryError;

/// ドメイン層におけるユーザー情報のリポジトリ定義
/// UserRepositoryの実装はinfrastructureで行う
pub trait UserRepository {
    /// Userデータを保存する
    fn create<'a>(
        &'a self,
        tx: &'a mut Transaction<'_, MySql>,
        user_builder: &'a UserBuilder,
    ) -> Pin<Box<dyn Future<Output = Result<UserId, RepositoryError>> + Send + 'a>>;

    /// UserIdによってUserデータを取得する
    fn find_by_id<'a>(
        &'a self,
        id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<User, RepositoryError>> + Send + 'a>>;

    /// emailによってUserデータを取得する
    fn find_by_email<'a>(
        &'a self,
        email: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<User, RepositoryError>> + Send + 'a>>;

    /// Userデータを変更する
    fn update<'a>(
        &'a self,
        user: &'a User,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>>;

    /// Userデータを削除する
    fn delete<'a>(
        &'a self,
        id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>>;

    /// Userが存在するか確認する
    fn is_exist<'a>(
        &'a self,
        email: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, RepositoryError>> + Send + 'a>>;
}
