use std::time::{Duration, SystemTime, UNIX_EPOCH};

use httpdate::HttpDate;

/// Get a [`HttpDate`] from unix time in seconds.
pub fn http_date_from_unix_time(secs: u64) -> HttpDate {
    HttpDate::from(system_time_from_unix_time(secs))
}

/// Get [`SystemTime`] from unix time in seconds.
fn system_time_from_unix_time(secs: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(secs)
}
