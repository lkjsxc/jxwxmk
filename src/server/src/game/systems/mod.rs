mod achievements;
mod actions;
mod barriers;
mod crafting;
mod death;
mod interaction;
mod quests;
mod spawning;
mod survival;

#[cfg(test)]
mod actions_test;

pub use achievements::evaluate_achievements;
pub use barriers::enforce_barriers;
pub use crafting::handle_craft;
pub use death::handle_deaths;
pub use interaction::handle_input;
pub use quests::{accept_quest, tick_quests};
pub use spawning::tick_spawns;
pub use survival::tick_survival;
