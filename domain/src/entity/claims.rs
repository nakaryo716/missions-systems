use super::user_id::UserId;

/// JWTを示す型
#[derive(Debug, Clone)]
pub struct Claims {
    pub user_id: UserId,
    pub exp: usize,
}
