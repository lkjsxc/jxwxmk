use redis::{AsyncCommands, Client, RedisResult};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct RedisRateLimiter {
    client: Client,
    max_requests: usize,
    window_secs: u64,
}

impl RedisRateLimiter {
    pub fn new(redis_url: &str, max_requests: usize, window_secs: u64) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        Ok(RedisRateLimiter {
            client,
            max_requests,
            window_secs,
        })
    }

    pub async fn check_rate(&self,
        key: &str,
    ) -> RedisResult<bool> {
        let mut conn = self.client.get_async_connection().await?;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let window_start = now - self.window_secs;
        
        // Remove old entries
        let _: () = conn.zrembyscore(key, 0, window_start).await?;
        
        // Count current entries
        let count: usize = conn.zcard(key).await?;
        
        if count >= self.max_requests {
            return Ok(false);
        }
        
        // Add new entry
        let _: () = conn.zadd(key, now.to_string(), now as f64).await?;
        
        // Set expiry on the key
        let _: () = conn.expire(key, self.window_secs as usize).await?;
        
        Ok(true)
    }

    pub async fn get_remaining(&self,
        key: &str,
    ) -> RedisResult<usize> {
        let mut conn = self.client.get_async_connection().await?;
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let window_start = now - self.window_secs;
        
        // Remove old entries
        let _: () = conn.zrembyscore(key, 0, window_start).await?;
        
        // Count current entries
        let count: usize = conn.zcard(key).await?;
        
        Ok(self.max_requests.saturating_sub(count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_redis_rate_limiter() {
        // This test requires a running Redis instance
        // Skip if Redis is not available
        let limiter = match RedisRateLimiter::new("redis://127.0.0.1:6379", 5, 60) {
            Ok(l) => l,
            Err(_) => {
                println!("Skipping test - Redis not available");
                return;
            }
        };

        let key = "test_key";
        
        // Should allow 5 requests
        for _ in 0..5 {
            assert!(limiter.check_rate(key).await.unwrap());
        }
        
        // 6th request should be denied
        assert!(!limiter.check_rate(key).await.unwrap());
    }
}
