use crate::game::events::EngineEvent;
use crate::game::messages::OutboundMessage;
use crate::game::systems::{
    accept_quest, enforce_barriers, evaluate_achievements, handle_craft, handle_deaths, handle_input,
    tick_quests, tick_spawns, tick_survival,
};
use crate::game::{engine_world, GameEngine};
use crate::persistence::save_player;

pub fn run_tick(engine: &mut GameEngine) {
    engine.tick = engine.tick.wrapping_add(1);
    let delta_seconds = 1.0 / engine.config.server.tick_rate.max(1.0);
    let mut outbox: Vec<OutboundMessage> = Vec::new();

    while let Some(event) = engine.input_queue.pop_front() {
        handle_event(engine, event, delta_seconds, &mut outbox);
    }

    let active_chunks = engine_world::update_interest_sets(&mut engine.world, &engine.config, engine.tick, &mut outbox);
    engine.world.active_chunks = active_chunks;

    tick_survival(&mut engine.world, &engine.config, delta_seconds);
    enforce_barriers(&mut engine.world);
    tick_quests(&mut engine.world, &mut outbox);
    evaluate_achievements(&mut engine.world, &engine.config, &mut outbox);
    handle_deaths(&mut engine.world, &mut outbox);
    tick_spawns(&mut engine.world, &engine.config, delta_seconds, engine.tick);

    engine_world::build_entity_deltas(&engine.world, &engine.config, &mut outbox);

    flush_outbox(engine, outbox);
    maybe_save_players(engine);
}

pub fn enqueue_event(engine: &mut GameEngine, event: EngineEvent) {
    if engine.input_queue.len() < engine.config.server.input_queue_limit {
        engine.input_queue.push_back(event);
    }
}

fn handle_event(engine: &mut GameEngine, event: EngineEvent, delta_seconds: f32, outbox: &mut Vec<OutboundMessage>) {
    match event {
        EngineEvent::Input { player_id, input } => {
            handle_input(&mut engine.world, &engine.config, player_id, input, engine.tick, delta_seconds, outbox);
        }
        EngineEvent::Spawn { player_id, settlement_id } => {
            handle_spawn(engine, player_id, settlement_id);
        }
        EngineEvent::Craft { player_id, recipe } => {
            let _ = handle_craft(&mut engine.world, &engine.config, player_id, &recipe);
        }
        EngineEvent::Trade { .. } => {}
        EngineEvent::NpcAction { .. } => {}
        EngineEvent::AcceptQuest { player_id, quest_id } => {
            accept_quest(&mut engine.world, &engine.config, player_id, &quest_id, outbox);
        }
        EngineEvent::SelectSlot { player_id, slot } => {
            if let Some(player) = engine.world.get_player_mut(&player_id) {
                if slot < player.inventory.slots.len() {
                    player.active_slot = slot;
                }
            }
        }
        EngineEvent::SwapSlots { player_id, from, to } => {
            if let Some(player) = engine.world.get_player_mut(&player_id) {
                let _ = player.inventory.swap(from, to);
            }
        }
        EngineEvent::Name { player_id, name } => {
            if let Some(player) = engine.world.get_player_mut(&player_id) {
                if !name.trim().is_empty() {
                    player.username = name.trim().chars().take(24).collect();
                }
            }
        }
        EngineEvent::Join { .. } | EngineEvent::Leave { .. } => {}
    }
}

fn handle_spawn(engine: &mut GameEngine, player_id: crate::game::entities::PlayerId, settlement_id: Option<String>) {
    let settlement = settlement_id
        .as_ref()
        .and_then(|id| engine.world.settlements.get(id))
        .or_else(|| engine.world.settlements.values().next());
    let (spawn_x, spawn_y) = settlement
        .map(|settlement| (settlement.spawn_x, settlement.spawn_y))
        .unwrap_or((0.0, 0.0));

    let Some(player) = engine.world.get_player_mut(&player_id) else {
        return;
    };

    player.x = spawn_x;
    player.y = spawn_y;

    player.spawned = true;
    player.health = engine.config.balance.player.max_health;
    player.hunger = 100.0;
    player.temperature = engine.config.survival.neutral_temp;
    player.thirst = 100.0;
}

fn flush_outbox(engine: &mut GameEngine, outbox: Vec<OutboundMessage>) {
    for (player_id, msg) in outbox {
        if let Some(session) = engine.sessions.get(&player_id) {
            let _ = session.do_send(msg);
        }
    }
}

fn maybe_save_players(engine: &GameEngine) {
    if engine.save_interval_ticks == 0 {
        return;
    }
    if engine.tick % engine.save_interval_ticks != 0 {
        return;
    }
    let pool = engine.db.clone();
    for player in engine.world.players.values() {
        let pool = pool.clone();
        let player = player.clone();
        actix::spawn(async move {
            let _ = save_player(&pool, &player).await;
        });
    }
}
