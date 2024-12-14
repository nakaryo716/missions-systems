use std::{future::Future, pin::Pin};

use crate::entity::{
    daily_mission::DailyMission, daily_mission_builder::DailyMissionBuilder,
    daily_mission_id::DailyMissionId, user_id::UserId,
};

use super::repository_error::RepositoryError;

/// ドメイン層におけるデイリーミッションのリポジトリ定義
/// DailyMissionRepositoryの実装はinfrastructureで行う
pub trait DailyMissionRepository {
    /// DailyMissionデータを保存する
    fn create(
        &self,
        builder: &DailyMissionBuilder,
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
    fn set_complete_true(
        &self,
        mission_id: &DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>>;

    /// 指定されたDailyMissionデータ一つを削除する
    fn delete(
        &self,
        mission_id: &DailyMissionId,
    ) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + Send + 'static>>;
}
