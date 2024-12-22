use std::{future::Future, pin::Pin};

use crate::entity::{user_exp::UserExp, user_id::UserId};

use super::repository_error::RepositoryError;

/// ドメイン層におけるユーザーの経験値情報のリポジトリ定義
/// UserExpRepositoryの実装はinfrastructureで行う
pub trait UserExpRepository {
    /// UserExpを初期化(データベースに登録する)
    /// そのため各ユーザー
    fn init_exp(
        &self,
        user_id: &UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>>;

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
