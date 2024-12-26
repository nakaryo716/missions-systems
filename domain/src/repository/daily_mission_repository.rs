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
    fn create(
        &self,
        builder: &DailyMission,
    ) -> Pin<Box<dyn Future<Output = Result<DailyMissionId, RepositoryError>> + Send + 'static>>;

    /// DailyMissionIdを使用して一つのDailyMissionデータを取得する
    fn find_by_id(
        &self,
        mission_id: &DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<DailyMission, RepositoryError>> + Send + 'static>>;

    /// ユーザーのDailyMissionデータ**すべて**を取得する
    fn find_by_user_id(
        &self,
        user_id: &UserId,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<DailyMission>, RepositoryError>> + Send + 'static>>;

    /// DailyMissionデータを変更する
    /// DailyMissionIdは引数のDailyMissionから参照する
    fn update(
        &self,
        mission: &DailyMission,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>>;

    /// DailyMissionのis_completeフィールドをfalseからtrueにセットする
    fn set_complete_true<'a>(
        &'a self,
        tx: &'a mut Transaction<'_, MySql>,
        mission_id: &DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'a>>;

    /// 指定されたDailyMissionデータ一つを削除する
    fn delete(
        &self,
        mission_id: &DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>>;
}
