use tokio::sync::{broadcast, mpsc};
use tokio::time::{self, Duration};

use crate::world::input::InputEvent;
use crate::world::state::{PlayerState, WorldState};

#[derive(Debug, Clone)]
pub struct ServerSnapshot {
    pub server_tick: u64,
}

pub fn run_tick_loop(
    tick_hz: u64,
    mut input_rx: mpsc::Receiver<InputEvent>,
    snapshot_tx: broadcast::Sender<ServerSnapshot>,
) {
    let tick_duration = Duration::from_millis(1000 / tick_hz.max(1));

    tokio::spawn(async move {
        let mut interval = time::interval(tick_duration);
        let mut state = WorldState::default();

        loop {
            interval.tick().await;

            while let Ok(event) = input_rx.try_recv() {
                let player = state
                    .players
                    .entry(event.session_id)
                    .or_insert_with(PlayerState::default);
                player.last_seq = player.last_seq.max(event.seq);
            }

            state.tick = state.tick.saturating_add(1);
            let _ = snapshot_tx.send(ServerSnapshot {
                server_tick: state.tick,
            });
        }
    });
}
