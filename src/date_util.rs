use chrono::{Duration, Local, NaiveDate};

pub fn today_string() -> String {
    Local::now().format("%Y-%m-%d %H:%M").to_string()
}

pub fn yesterday_string() -> String {
    let today = Local::now();
    let one_day = Duration::days(1);
    let yesterday = today - one_day;

    yesterday.format("%Y-%m-%d %H:%M").to_string()
}

pub fn is_valid_date_string(date_string: &String) -> bool {
    NaiveDate::parse_from_str(date_string, "%Y-%m-%d %H:%M").is_ok()
}
