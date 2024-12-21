use sqlx::{mysql::MySqlRow, FromRow, Row};

use super::{daily_mission_id::DailyMissionId, user_id::UserId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyMission {
    pub user_id: UserId,
    pub mission_id: DailyMissionId,
    pub title: String,
    pub description: Option<String>,
    pub is_complete: bool,
}

impl FromRow<'_, MySqlRow> for DailyMission {
    fn from_row(row: &'_ MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user_id: UserId(row.try_get("user_id")?),
            mission_id: DailyMissionId(row.try_get("mission_id")?),
            title: row.try_get("title")?,
            description: row.try_get("descriptions")?,
            is_complete: row.try_get("is_complete")?,
        })
    }
}
