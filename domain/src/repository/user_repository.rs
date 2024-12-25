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
    fn find_by_id(
        &self,
        id: &UserId,
    ) -> Pin<Box<dyn Future<Output = Result<User, RepositoryError>> + Send + 'static>>;

    /// emailによってUserデータを取得する
    fn find_by_email(
        &self,
        email: &str,
    ) -> Pin<Box<dyn Future<Output = Result<User, RepositoryError>> + Send + 'static>>;

    /// Userデータを変更する
    fn update(
        &self,
        user: &User,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>>;

    /// Userデータを削除する
    fn delete(
        &self,
        id: &UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>>;

    /// Userが存在するか確認する
    fn is_exist(
        &self,
        email: &str,
    ) -> Pin<Box<dyn Future<Output = Result<bool, RepositoryError>> + Send + 'static>>;
}
