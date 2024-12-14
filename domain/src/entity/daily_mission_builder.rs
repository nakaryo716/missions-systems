use super::user_id::UserId;

#[derive(Debug, Clone)]
pub struct DailyMissionBuilder {
    pub user_id: UserId,
    pub title: String,
    pub description: Option<String>,
}
