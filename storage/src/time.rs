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
        format!("{} seconds ago", seconds)
    } else if seconds < 3600 {
        let minutes = seconds / 60;
        format!("{} minutes ago", minutes)
    } else if seconds < 86400 {
        let hours = seconds / 3600;
        format!("{} hours ago", hours)
    } else if seconds < 2_592_000 {
        let days = seconds / 86400;
        format!("{} days ago", days)
    } else if seconds < 31_536_000 {
        let months = seconds / 2_592_000;
        format!("{} months ago", months)
    } else {
        let years = seconds / 31_536_000;
        format!("{} years ago", years)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    // ---------- tests for convert_to_jalali ----------
    #[test]
    fn test_convert_to_jalali_epoch() {
        // Unix epoch (1970-01-01) → expected Jalali date: 1348/10/11
        assert_eq!(convert_to_jalali(0), "1348/10/11");
    }

    #[test]
    fn test_convert_to_jalali_known_date() {
        // 2020-01-01 00:00:00 UTC → 1398/10/11
        assert_eq!(convert_to_jalali(1577836800), "1398/10/11");
    }

    #[test]
    fn test_convert_to_jalali_format() {
        let result = convert_to_jalali(1_000_000_000); // 2001-09-09
        assert_eq!(result.len(), 10);
        assert_eq!(&result[4..5], "/");
        assert_eq!(&result[7..8], "/");
        assert!(result.chars().all(|c| c.is_ascii_digit() || c == '/'));
    }

    // ---------- tests for time_ago (pure function) ----------
    #[test]
    fn test_time_ago_seconds() {
        assert_eq!(time_ago(0), "0 seconds ago");
        assert_eq!(time_ago(30), "30 seconds ago");
        assert_eq!(time_ago(59), "59 seconds ago");
    }

    #[test]
    fn test_time_ago_minutes() {
        assert_eq!(time_ago(60), "1 minutes ago");
        assert_eq!(time_ago(120), "2 minutes ago");
        assert_eq!(time_ago(3599), "59 minutes ago");
    }

    #[test]
    fn test_time_ago_hours() {
        assert_eq!(time_ago(3600), "1 hours ago");
        assert_eq!(time_ago(7200), "2 hours ago");
        assert_eq!(time_ago(86399), "23 hours ago");
    }

    #[test]
    fn test_time_ago_days() {
        assert_eq!(time_ago(86400), "1 days ago");
        assert_eq!(time_ago(172800), "2 days ago");
        assert_eq!(time_ago(2_591_999), "29 days ago"); // just below 30 days
    }

    #[test]
    fn test_time_ago_months() {
        // 30-day month assumption
        assert_eq!(time_ago(2_592_000), "1 months ago");
        assert_eq!(time_ago(2_592_000 * 2), "2 months ago");
        assert_eq!(time_ago(31_535_999), "12 months ago"); // 12 * 30 days
    }

    #[test]
    fn test_time_ago_years() {
        assert_eq!(time_ago(31_536_000), "1 years ago");
        assert_eq!(time_ago(63_072_000), "2 years ago");
        assert_eq!(time_ago(100_000_000), "3 years ago"); // ~3.17 years → 3
    }

    // ---------- tests for distance_calculator (uses Utc::now) ----------
    #[test]
    fn test_distance_calculator_recent_past() {
        let now = Utc::now().timestamp();
        let five_sec_ago = now - 5;
        let result = distance_calculator(five_sec_ago);
        // Should be "5 seconds ago" or something similar; allow a small tolerance
        // Because the clock may tick between getting now and calling the function,
        // we check that it contains "seconds ago" and a number close to 5.
        assert!(result.ends_with(" seconds ago"));
        let parts: Vec<&str> = result.split(' ').collect();
        assert_eq!(parts.len(), 3); // ["5", "seconds", "ago"]
        let num = parts[0].parse::<i64>().unwrap();
        // The difference should be between 4 and 6 (if the test runs quickly)
        assert!((4..=6).contains(&num));
    }

    #[test]
    fn test_distance_calculator_recent_future() {
        let now = Utc::now().timestamp();
        let five_sec_ahead = now + 5;
        let result = distance_calculator(five_sec_ahead);
        // Should be negative seconds, e.g., "-5 seconds ago"
        assert!(result.ends_with(" seconds ago"));
        let parts: Vec<&str> = result.split(' ').collect();
        assert_eq!(parts.len(), 3);
        let num = parts[0].parse::<i64>().unwrap();
        // Negative number: difference is now - future = -5, so around -4 to -6
        assert!((-6..=-4).contains(&num));
    }

    #[test]
    fn test_distance_calculator_far_past() {
        // timestamp of 1970-01-01 → huge difference → years
        let result = distance_calculator(0);
        assert!(result.ends_with(" years ago"));
        let parts: Vec<&str> = result.split(' ').collect();
        assert_eq!(parts.len(), 3);
        let years = parts[0].parse::<i64>().unwrap();
        // Should be around 55+ (depending on current year)
        assert!(years > 50);
    }

    #[test]
    fn test_distance_calculator_exact_minute() {
        let now = Utc::now().timestamp();
        let minute_ago = now - 60;
        let result = distance_calculator(minute_ago);
        // Should be "1 minutes ago" or maybe "0" if clock shifted; we can check it contains "minutes ago"
        assert!(result.ends_with(" minutes ago"));
        let parts: Vec<&str> = result.split(' ').collect();
        let num = parts[0].parse::<i64>().unwrap();
        // Could be 1, but allow 0 or 1 due to timing
        assert!((0..=2).contains(&num));
    }
}
