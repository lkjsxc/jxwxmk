use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    window: Duration,
    limit: u32,
    entries: HashMap<String, (Instant, u32)>,
}

impl RateLimiter {
    pub fn new(limit: u32, window: Duration) -> Self {
        Self {
            window,
            limit,
            entries: HashMap::new(),
        }
    }

    pub fn allow(&mut self, key: &str) -> bool {
        let now = Instant::now();
        let entry = self.entries.entry(key.to_string()).or_insert((now, 0));
        if now.duration_since(entry.0) > self.window {
            *entry = (now, 0);
        }
        if entry.1 >= self.limit {
            return false;
        }
        entry.1 += 1;
        true
    }
}
