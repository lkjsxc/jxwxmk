pub mod config;
pub mod http;
pub mod ws;

use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};

use crate::world::{input::InputEvent, tick::ServerSnapshot};

#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    pub input_tx: mpsc::Sender<InputEvent>,
    pub snapshot_tx: broadcast::Sender<ServerSnapshot>,
    pub tick_hz: u64,
}

impl AppState {
    pub fn new(
        input_tx: mpsc::Sender<InputEvent>,
        snapshot_tx: broadcast::Sender<ServerSnapshot>,
        tick_hz: u64,
    ) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                input_tx,
                snapshot_tx,
                tick_hz,
            }),
        }
    }

    pub fn input_tx(&self) -> &mpsc::Sender<InputEvent> {
        &self.inner.input_tx
    }

    pub fn snapshot_tx(&self) -> &broadcast::Sender<ServerSnapshot> {
        &self.inner.snapshot_tx
    }

    pub fn tick_hz(&self) -> u64 {
        self.inner.tick_hz
    }
}
