use super::user_id::UserId;

#[derive(Debug, Clone)]
pub struct UserBuilder {
    pub user_id: UserId,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
}
