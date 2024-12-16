use super::user_id::UserId;

/// JWTを示す型
#[derive(Debug, Clone)]
pub struct Claims {
    pub user_id: UserId,
    pub exp: usize,
}

impl Claims {
    pub fn new(user_id: UserId, exp: usize) -> Self {
        Self { user_id, exp }
    }
}
