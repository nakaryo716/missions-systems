use sqlx::{mysql::MySqlRow, FromRow, Row};

use super::user_id::UserId;

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: UserId,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
}

impl FromRow<'_, MySqlRow> for User {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user_id: UserId(row.try_get("user_id")?),
            user_name: row.try_get("user_name")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
        })
    }
}
