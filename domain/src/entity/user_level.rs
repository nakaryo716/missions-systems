use serde::Serialize;

use crate::service::level_convert::LevelConvert;

use super::{user_exp::UserExp, user_id::UserId};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLevel {
    pub user_id: UserId,
    pub experience_points: i64,
    pub level: u32,
    pub remain: Option<u32>,
}

impl UserLevel {
    pub fn new<T>(user_exp: UserExp, converter: &T) -> Self
    where
        T: LevelConvert,
    {
        // 現在のレベルとレベルアップに必要な経験値量を計算
        let (level, remain) = converter.to_level_with_remain(user_exp.experience_points);
        Self {
            user_id: user_exp.user_id,
            experience_points: user_exp.experience_points,
            level,
            remain,
        }
    }
}
