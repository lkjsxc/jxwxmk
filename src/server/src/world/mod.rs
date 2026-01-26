pub mod input;
pub mod protocol;
pub mod state;
pub mod tick;

use tokio::sync::{broadcast, mpsc};

use crate::world::input::InputEvent;
use crate::world::tick::{run_tick_loop, ServerSnapshot};

pub struct WorldRunner;

impl WorldRunner {
    pub fn start(tick_hz: u64) -> (mpsc::Sender<InputEvent>, broadcast::Sender<ServerSnapshot>) {
        let (input_tx, input_rx) = mpsc::channel(1024);
        let (snapshot_tx, _) = broadcast::channel(64);

        run_tick_loop(tick_hz, input_rx, snapshot_tx.clone());

        (input_tx, snapshot_tx)
    }
}
