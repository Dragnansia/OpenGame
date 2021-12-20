use std::time::SystemTime;

pub fn current_time() -> SystemTime {
    SystemTime::now()
}

/// Return the duration between current_time call and now
/// with `elapsed` function of SystemTime
pub fn get_duration(timer: &SystemTime) -> u64 {
    if let Ok(seconds) = timer.elapsed() {
        seconds.as_secs()
    } else {
        u64::MAX
    }
}
