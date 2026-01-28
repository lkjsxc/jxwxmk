use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct GameMetrics {
    inner: Arc<MetricsInner>,
}

struct MetricsInner {
    active_players: AtomicUsize,
    active_chunks: AtomicUsize,
    tick_duration_ms: AtomicU64,
    tick_count: AtomicU64,
    messages_sent: AtomicU64,
    messages_received: AtomicU64,
}

impl GameMetrics {
    pub fn new() -> Self {
        GameMetrics {
            inner: Arc::new(MetricsInner {
                active_players: AtomicUsize::new(0),
                active_chunks: AtomicUsize::new(0),
                tick_duration_ms: AtomicU64::new(0),
                tick_count: AtomicU64::new(0),
                messages_sent: AtomicU64::new(0),
                messages_received: AtomicU64::new(0),
            }),
        }
    }

    pub fn set_active_players(&self,
        count: usize,
    ) {
        self.inner.active_players.store(count, Ordering::Relaxed);
    }

    pub fn set_active_chunks(&self,
        count: usize,
    ) {
        self.inner.active_chunks.store(count, Ordering::Relaxed);
    }

    pub fn record_tick(&self,
        duration_ms: u64,
    ) {
        self.inner.tick_duration_ms.store(duration_ms, Ordering::Relaxed);
        self.inner.tick_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_messages_sent(&self,
        count: u64,
    ) {
        self.inner.messages_sent.fetch_add(count, Ordering::Relaxed);
    }

    pub fn increment_messages_received(&self,
        count: u64,
    ) {
        self.inner.messages_received.fetch_add(count, Ordering::Relaxed);
    }

    pub fn get_active_players(&self,
    ) -> usize {
        self.inner.active_players.load(Ordering::Relaxed)
    }

    pub fn get_active_chunks(&self,
    ) -> usize {
        self.inner.active_chunks.load(Ordering::Relaxed)
    }

    pub fn get_tick_duration_ms(&self,
    ) -> u64 {
        self.inner.tick_duration_ms.load(Ordering::Relaxed)
    }

    pub fn get_tick_count(&self,
    ) -> u64 {
        self.inner.tick_count.load(Ordering::Relaxed)
    }

    pub fn get_messages_sent(&self,
    ) -> u64 {
        self.inner.messages_sent.load(Ordering::Relaxed)
    }

    pub fn get_messages_received(&self,
    ) -> u64 {
        self.inner.messages_received.load(Ordering::Relaxed)
    }

    pub fn render_prometheus(&self) -> String {
        format!(
            r#"# HELP jxwxmk_active_players Number of active players
# TYPE jxwxmk_active_players gauge
jxwxmk_active_players {}

# HELP jxwxmk_active_chunks Number of active chunks
# TYPE jxwxmk_active_chunks gauge
jxwxmk_active_chunks {}

# HELP jxwxmk_tick_duration_seconds Tick duration in seconds
# TYPE jxwxmk_tick_duration_seconds gauge
jxwxmk_tick_duration_seconds {}

# HELP jxwxmk_tick_count Total number of ticks processed
# TYPE jxwxmk_tick_count counter
jxwxmk_tick_count {}

# HELP jxwxmk_messages_sent_total Total messages sent to clients
# TYPE jxwxmk_messages_sent_total counter
jxwxmk_messages_sent_total {}

# HELP jxwxmk_messages_received_total Total messages received from clients
# TYPE jxwxmk_messages_received_total counter
jxwxmk_messages_received_total {}
"#,
            self.get_active_players(),
            self.get_active_chunks(),
            self.get_tick_duration_ms() as f64 / 1000.0,
            self.get_tick_count(),
            self.get_messages_sent(),
            self.get_messages_received(),
        )
    }
}

impl Default for GameMetrics {
    fn default() -> Self {
        Self::new()
    }
}
