use chrono::{DateTime, Local};

pub fn now() -> DateTime<Local> {
    Local::now()
}

pub fn now_timestamp_millis() -> i64 {
    Local::now().timestamp_millis()
}

pub fn now_timestamp_secs() -> i64 {
    Local::now().timestamp()
}

pub fn format_time(dt: &DateTime<Local>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

pub fn iso8601() -> String {
    Local::now().to_rfc3339()
}

pub fn daily_key() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}
