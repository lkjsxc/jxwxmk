use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

pub struct EngineMetrics {
    pub tick_duration_ms: AtomicU64,
    pub tick_overruns: AtomicU64,
    pub active_players: AtomicUsize,
    pub active_chunks: AtomicUsize,
    pub input_queue_len: AtomicUsize,
    pub input_dropped: AtomicU64,
}

impl EngineMetrics {
    pub fn new() -> Self {
        Self {
            tick_duration_ms: AtomicU64::new(0),
            tick_overruns: AtomicU64::new(0),
            active_players: AtomicUsize::new(0),
            active_chunks: AtomicUsize::new(0),
            input_queue_len: AtomicUsize::new(0),
            input_dropped: AtomicU64::new(0),
        }
    }

    pub fn to_prometheus(&self) -> String {
        format!(
            "# HELP jxwxmk_active_players Number of active players\n# TYPE jxwxmk_active_players gauge\njxwxmk_active_players {}\n\
             # HELP jxwxmk_active_chunks Number of active chunks\n# TYPE jxwxmk_active_chunks gauge\njxwxmk_active_chunks {}\n\
             # HELP jxwxmk_engine_input_queue_len Engine input queue length\n# TYPE jxwxmk_engine_input_queue_len gauge\njxwxmk_engine_input_queue_len {}\n",
            self.active_players.load(Ordering::Relaxed),
            self.active_chunks.load(Ordering::Relaxed),
            self.input_queue_len.load(Ordering::Relaxed)
        )
    }
}
