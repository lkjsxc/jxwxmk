use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct RateLimiter {
    limit: u32,
    window: Duration,
    buckets: std::sync::Arc<std::sync::Mutex<HashMap<IpAddr, Bucket>>>,
}

#[derive(Clone, Debug)]
struct Bucket {
    tokens: u32,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(limit: u32, window: Duration) -> Self {
        Self {
            limit,
            window,
            buckets: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    pub fn allow(&self, ip: IpAddr) -> bool {
        let mut buckets = self.buckets.lock().expect("rate limiter lock");
        let bucket = buckets.entry(ip).or_insert_with(|| Bucket {
            tokens: self.limit,
            last_refill: Instant::now(),
        });
        let elapsed = bucket.last_refill.elapsed();
        if elapsed >= self.window {
            bucket.tokens = self.limit;
            bucket.last_refill = Instant::now();
        }
        if bucket.tokens == 0 {
            return false;
        }
        bucket.tokens -= 1;
        true
    }
}
