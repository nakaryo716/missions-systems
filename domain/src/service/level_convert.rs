// 経験値をレベルに変換するトレイト
pub trait LevelConvert {
    fn to_level(&self, experience_point: u64) -> i32;
}
