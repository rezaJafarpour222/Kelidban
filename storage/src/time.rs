use chrono::Utc;
use jalali_calendar::JalaliDateTime;
pub fn convert_to_jalali(timestamp: i64) -> String {
    let jalali = JalaliDateTime::from_unix_timestamp(timestamp).unwrap();

    jalali.format("%Y/%m/%d")
}
pub fn distance_calculator(timestmp: i64) -> String {
    let now = Utc::now().timestamp();
    let difference = now - timestmp;
    time_ago(difference)
}

fn time_ago(seconds: i64) -> String {
    if seconds < 60 {
        format!("{} قبل", seconds)
    } else if seconds < 3600 {
        let minutes = seconds / 60;
        format!("{} قبل", minutes)
    } else if seconds < 86400 {
        let hours = seconds / 3600;
        format!("{} قبل", hours)
    } else if seconds < 2_592_000 {
        let days = seconds / 86400;
        format!("{} قبل", days)
    } else if seconds < 31_536_000 {
        let months = seconds / 2_592_000;
        format!("{} قبل", months)
    } else {
        let years = seconds / 31_536_000;
        format!("{} قبل", years)
    }
}
#[cfg(test)]
mod tests {}
