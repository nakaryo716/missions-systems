// 経験値とレベル間での振る舞いを担うトレイト
pub trait LevelConvert {
    // 所持経験値をレベルに変換する
    // レベルアップに必要な経験値を計算するメソッド
    // 戻り値は(現在のレベル, レベルアップに必要な経験値量)
    fn to_level_with_remain(&self, experience_point: i64) -> (u32, Option<u32>);
}
