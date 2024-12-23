use serde::Serialize;

use super::{user::User, user_id::UserId};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user_id: UserId,
    pub user_name: String,
}

impl From<User> for UserInfo {
    fn from(value: User) -> Self {
        Self {
            user_id: value.user_id,
            user_name: value.user_name,
        }
    }
}
