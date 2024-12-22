use crate::service::level_convert::LevelConvert;

use super::{user_exp::UserExp, user_id::UserId};

#[derive(Debug, Clone)]
pub struct UserLevel {
    pub user_id: UserId,
    pub experience_points: i64,
    pub level: u32,
}

impl UserLevel {
    pub fn new<T>(user_exp: UserExp, converter: &T) -> Self
    where
        T: LevelConvert,
    {
        Self {
            user_id: user_exp.user_id,
            experience_points: user_exp.experience_points,
            level: converter.to_level(user_exp.experience_points),
        }
    }
}
