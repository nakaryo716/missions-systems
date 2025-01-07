use super::{user::User, user_id::UserId};

#[derive(Debug, Clone)]
pub struct UserBuilder {
    pub user_id: UserId,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            user_id: UserId("".to_string()),
            user_name: "".to_string(),
            email: "".to_string(),
            password_hash: "".to_string(),
        }
    }

    pub fn user_id(self, user_id: UserId) -> Self {
        Self {
            user_id,
            user_name: self.user_name,
            email: self.email,
            password_hash: self.password_hash,
        }
    }

    pub fn user_name(self, user_name: String) -> Self {
        Self {
            user_id: self.user_id,
            user_name,
            email: self.email,
            password_hash: self.password_hash,
        }
    }

    pub fn email(self, email: String) -> Self {
        Self {
            user_id: self.user_id,
            user_name: self.user_name,
            email,
            password_hash: self.password_hash,
        }
    }

    pub fn password_hash(self, password_hash: String) -> Self {
        Self {
            user_id: self.user_id,
            user_name: self.user_name,
            email: self.email,
            password_hash,
        }
    }

    pub fn build(self) -> User {
        User {
            user_id: self.user_id,
            user_name: self.user_name,
            email: self.email,
            password_hash: self.password_hash,
        }
    }
}
