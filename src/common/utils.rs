use chrono::{DateTime, Local};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_ts_iso8601() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let dt: DateTime<Local> = DateTime::<Local>::from(UNIX_EPOCH + now);
    dt.format("%Y-%m-%dT%H:%M:%S%.6f%:z").to_string()
}
