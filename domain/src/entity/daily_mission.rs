use super::user_id::UserId;

#[derive(Debug, Clone)]
pub struct DailyMission {
    pub user_id: UserId,
    pub title: String,
    pub description: Option<String>,
    pub is_complete: bool,
}
