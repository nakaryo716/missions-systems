use std::{future::Future, pin::Pin};

use sqlx::{MySql, Transaction};

use crate::entity::{
    daily_mission::DailyMission, daily_mission_id::DailyMissionId, user_id::UserId,
};

use super::repository_error::RepositoryError;

/// ドメイン層におけるデイリーミッションのリポジトリ定義
/// DailyMissionRepositoryの実装はinfrastructureで行う
pub trait DailyMissionRepository {
    /// DailyMissionデータを保存する
    fn create<'a>(
        &'a self,
        builder: &'a DailyMission,
    ) -> Pin<Box<dyn Future<Output = Result<DailyMissionId, RepositoryError>> + Send + 'a>>;

    /// ユーザーが登録したミッションがいくつあるのかカウントするメソッド
    fn count<'a>(
        &'a self,
        user_id: &'a UserId
    ) -> Pin<Box<dyn Future<Output = Result<i32, RepositoryError>> + Send + 'a>>;

    /// DailyMissionIdを使用して一つのDailyMissionデータを取得する
    fn find_by_id<'a>(
        &'a self,
        mission_id: &'a DailyMissionId,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<DailyMission, RepositoryError>> + Send + 'a>>;

    /// ユーザーのDailyMissionデータ**すべて**を取得する
    fn find_by_user_id<'a>(
        &'a self,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DailyMission>, RepositoryError>> + Send + 'a>>;

    /// DailyMissionデータを変更する
    /// DailyMissionIdは引数のDailyMissionから参照する
    fn update<'a>(
        &'a self,
        mission: &'a DailyMission,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>>;

    /// DailyMissionのis_completeフィールドをfalseからtrueにセットする
    fn set_complete_true<'a>(
        &'a self,
        tx: &'a mut Transaction<'_, MySql>,
        mission_id: &'a DailyMissionId,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>>;

    /// 指定されたDailyMissionデータ一つを削除する
    fn delete<'a>(
        &'a self,
        mission_id: &'a DailyMissionId,
        user_id: &'a UserId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>>;
}
