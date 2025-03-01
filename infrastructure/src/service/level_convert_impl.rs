use std::{collections::BTreeMap, sync::LazyLock};

use domain::service::level_convert::LevelConvert;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ExpTable {
    level: u32,
    exp: u32,
}

// CSVにある経験値テーブルをBTreeMapとして取得し、実行時は保持している
// 最適化していない状態で取り出し(get)は200ns程
static EXP_TABLE: LazyLock<BTreeMap<u32, u32>> = LazyLock::new(|| {
    let file_path = dotenvy::var("FILE_PATH").expect("Failed to get exp_table file path");
    let mut map = BTreeMap::new();
    let mut rdr = csv::Reader::from_path(file_path).expect("failed to open exp_table file");
    rdr.deserialize::<ExpTable>().for_each(|r| {
        let record = r.expect("failed to decode exp_table");
        map.insert(record.level, record.exp);
    });
    map
});

/// ユーザーの経験値をLevelに変換する
/// 指数関数的にレベルアップに必要な経験値が上昇する
/// 以下のような計算式になっている
/// Required_exp = 10 * Level^1.5
/// 1.5の倍率で経験値が必要になる
/// ただし必要経験値(Required_exp)は整数値に丸めておりcsv形式で取り出すことができる
/// csv は (current level, required exp to level up)　を意味している
#[derive(Debug, Clone)]
pub struct LevelConvertImpl;

impl LevelConvert for LevelConvertImpl {
    fn to_level_with_remain(&self, experience_point: i64) -> (u32, Option<u32>) {
        let mut current_level = 1;
        let mut required_exp = None;
        // 現在のレベルを調べる
        // 線形探索している[O(n)]
        for (l, exp) in EXP_TABLE.iter() {
            if (experience_point as u32) < *exp {
                required_exp = Some(*exp);
                break;
            }
            current_level = l + 1;
        }
        // レベルが100がMax
        // 100より大きい場合は100にする
        if current_level > 100 {
            current_level = 100;
        }
        // レベルアップに必要な経験値量を計算
        let remain = required_exp.map(|exp| exp - experience_point as u32);
        (current_level, remain)
    }
}

#[cfg(test)]
mod tests {
    use domain::service::level_convert::LevelConvert;
    use crate::service::level_convert_impl::LevelConvertImpl;

    #[test]
    fn test_with_level(){
        let a = LevelConvertImpl;
        assert_eq!(a.to_level_with_remain(0), (1, Some(10)));
        assert_eq!(a.to_level_with_remain(2), (1, Some(8)));
        assert_eq!(a.to_level_with_remain(10), (2, Some(18)));
        assert_eq!(a.to_level_with_remain(7623), (84, Some(76)));
        assert_eq!(a.to_level_with_remain(8396), (90, Some(142)));
        // レベルが100以上のケース
        assert_eq!(a.to_level_with_remain(10000), (100, None));
        assert_eq!(a.to_level_with_remain(120000), (100, None));
    }
}

