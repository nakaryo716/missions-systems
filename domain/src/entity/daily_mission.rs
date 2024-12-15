use super::{daily_mission_id::DailyMissionId, user_id::UserId};

#[derive(Debug, Clone)]
pub struct DailyMission {
    pub user_id: UserId,
    pub mission_id: DailyMissionId,
    pub title: String,
    pub description: Option<String>,
    pub is_complete: bool,
}
