use chrono::{NaiveDateTime, NaiveTime, TimeDelta};

// 実行時間を過ぎていたら翌日の時間、実行時間に到達していなかったらその日を返す
// ex1:
// event_time = 2024-12-25 18:00
// current_date_time = 2024-12-25 13:00
// -> return 2024-12-25 18:00
// ========================================
// ex2:
// event_time = 2024-12-25 18:00
// current_date_time = 2024-12-25 18:30
//                                ++++++
// -> return 2024-12-26 18:00
//                  ++++
pub(crate) fn next_event_date_time(
    current_time: NaiveDateTime,
    event_time: NaiveTime,
) -> NaiveDateTime {
    // NaiveDateTime型に変換し現在の日付と時間をそれぞれ取得
    // let current = current_time.naive_utc();
    let current_date = current_time.date();
    let current_time = current_time.time();

    if current_time > event_time {
        NaiveDateTime::new(current_date, event_time) + chrono::Duration::days(1)
    } else {
        NaiveDateTime::new(current_date, event_time)
    }
}

// 残り時間を計算する
pub(crate) fn remain(
    current_date_time: NaiveDateTime,
    event_date_time: NaiveDateTime,
) -> TimeDelta {
    event_date_time - current_date_time
}
