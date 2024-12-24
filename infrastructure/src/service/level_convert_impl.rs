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
/// Level0から1になるまでに10point必要で、それ以降、レベルアップするのに
/// 1.5の倍率で経験値が必要になる
/// ただし必要経験値(Required_exp)は整数値に丸めておりcsv形式で取り出すことができる
#[derive(Debug, Clone)]
pub struct LevelConvertImpl;

impl LevelConvert for LevelConvertImpl {
    fn to_level(&self, experience_point: i64) -> u32 {
        let table = &EXP_TABLE;
        let mut level = 0;
        for (l, exp) in table.iter() {
            if (experience_point as u32) < *exp {
                break;
            }
            level = *l;
        }
        level
    }
}

#[cfg(test)]
mod tests {
    use domain::service::level_convert::LevelConvert;

    use crate::service::level_convert_impl::LevelConvertImpl;

    #[test]
    fn test_level() {
        let level = LevelConvertImpl.to_level(2500);
        assert_eq!(level, 39);
    }

    #[test]
    fn test_level_boundary() {
        let converter = LevelConvertImpl;

        // 境界値付近をテスト
        assert_eq!(converter.to_level(9), 0); // 経験値が10未満の場合
        assert_eq!(converter.to_level(10), 1); // 経験値がちょうど10のとき

        // 高レベルへの境界値
        assert_eq!(converter.to_level(9849), 98); // 経験値が99に届かない場合
        assert_eq!(converter.to_level(9850), 99); // 経験値がちょうど99になる場合

        // 極端な値
        assert_eq!(converter.to_level(0), 0); // 最小値
        assert_eq!(converter.to_level(1_000_000), 100); // 超高経験値
    }
}
