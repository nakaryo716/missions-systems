// 経験値をレベルに変換するトレイト
pub trait LevelConvert {
    fn to_level(&self, experience_point: i64) -> u32;
}
