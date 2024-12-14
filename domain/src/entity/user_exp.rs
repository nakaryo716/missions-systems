use super::user_id::UserId;

#[derive(Debug, Clone)]
pub struct UserExp {
    pub user_id: UserId,
    pub experience_points: u64,
}
