use std::{future::Future, pin::Pin};

use sqlx::{MySql, Transaction};

use crate::entity::{user_exp::UserExp, user_id::UserId};

use super::repository_error::RepositoryError;

/// ドメイン層におけるユーザーの経験値情報のリポジトリ定義
/// UserExpRepositoryの実装はinfrastructureで行う
pub trait UserExpRepository {
    /// UserExpを初期化(データベースに登録する)
    /// そのため各ユーザー
    fn init_exp<'a>(
        &'a self,
        tx: &'a mut Transaction<'_, MySql>,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>>;

    /// UserExpを取得する
    fn find_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Pin<Box<dyn Future<Output = Result<UserExp, RepositoryError>> + Send + 'static>>;

    /// UserExpの経験値を増加(変更)させる
    fn add_exp(
        &self,
        user_id: &UserId,
        additional_exp: i64,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>>;
}
