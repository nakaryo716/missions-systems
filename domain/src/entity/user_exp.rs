use sqlx::{mysql::MySqlRow, FromRow, Row};

use super::user_id::UserId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserExp {
    pub user_id: UserId,
    pub experience_points: i64,
}

impl FromRow<'_, MySqlRow> for UserExp {
    fn from_row(row: &'_ MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user_id: UserId(row.try_get("user_id")?),
            experience_points: row.try_get("experience_points")?,
        })
    }
}
