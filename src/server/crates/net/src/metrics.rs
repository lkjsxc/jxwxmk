use prometheus::{Registry, Counter, Gauge, Histogram, HistogramOpts, TextEncoder, Encoder};

#[derive(Clone)]
pub struct Metrics {
    registry: Registry,
    tick_duration: Histogram,
    tick_overrun: Counter,
    active_players: Gauge,
    active_chunks: Gauge,
    input_queue_len: Gauge,
    input_dropped: Counter,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let tick_duration = Histogram::with_opts(
            HistogramOpts::new(
                "jxwxmk_tick_duration_seconds",
                "Time spent processing a tick"
            )
            .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0])
        ).unwrap();

        let tick_overrun = Counter::new(
            "jxwxmk_tick_overrun_total",
            "Number of ticks that exceeded target duration"
        ).unwrap();

        let active_players = Gauge::new(
            "jxwxmk_active_players",
            "Number of connected players"
        ).unwrap();

        let active_chunks = Gauge::new(
            "jxwxmk_active_chunks",
            "Number of active chunks"
        ).unwrap();

        let input_queue_len = Gauge::new(
            "jxwxmk_engine_input_queue_len",
            "Current length of input queue"
        ).unwrap();

        let input_dropped = Counter::new(
            "jxwxmk_engine_input_dropped_total",
            "Number of dropped input events"
        ).unwrap();

        registry.register(Box::new(tick_duration.clone())).unwrap();
        registry.register(Box::new(tick_overrun.clone())).unwrap();
        registry.register(Box::new(active_players.clone())).unwrap();
        registry.register(Box::new(active_chunks.clone())).unwrap();
        registry.register(Box::new(input_queue_len.clone())).unwrap();
        registry.register(Box::new(input_dropped.clone())).unwrap();

        Self {
            registry,
            tick_duration,
            tick_overrun,
            active_players,
            active_chunks,
            input_queue_len,
            input_dropped,
        }
    }

    pub fn gather(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap_or_default()
    }

    pub fn observe_tick_duration(&self, duration_secs: f64) {
        self.tick_duration.observe(duration_secs);
    }

    pub fn inc_tick_overrun(&self) {
        self.tick_overrun.inc();
    }

    pub fn set_active_players(&self, count: i64) {
        self.active_players.set(count as f64);
    }

    pub fn set_active_chunks(&self, count: i64) {
        self.active_chunks.set(count as f64);
    }

    pub fn set_input_queue_len(&self, len: i64) {
        self.input_queue_len.set(len as f64);
    }

    pub fn inc_input_dropped(&self) {
        self.input_dropped.inc();
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
