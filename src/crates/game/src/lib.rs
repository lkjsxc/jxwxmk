pub mod events;
pub mod engine;
pub mod metrics;
pub mod interest;
pub mod deltas;
mod tests;

pub use events::GameEvent;
pub use engine::{GameEngine, ClientMessage, OutboundMessage, GetMetrics};

pub fn init() {}