use chrono::Utc;
use chrono_tz::Asia::Kolkata;

pub fn school_time(opening_time: i32, closing_time: i32) -> bool {
    let utc = Utc::now();
    let ist = utc.with_timezone(&Kolkata).format("%H").to_string();
    let ist: i32 = ist.trim().parse().expect("Failed to parse time as i32");

    if ist > opening_time && ist <= closing_time {
        return true;
    } else {
        return false;
    }
}
pub fn date() -> i32 {
    let utc = Utc::now();
    let ist = utc.with_timezone(&Kolkata).format("%d").to_string();
    let ist: i32 = ist.trim().parse().expect("Failed to parse time as i32");
    ist
}