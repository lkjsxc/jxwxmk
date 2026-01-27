use chrono::{DateTime, Utc};

pub fn clamp_f32(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}
