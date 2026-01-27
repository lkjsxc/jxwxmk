use crate::config::Config;

use crate::game::entities::PlayerId;
use crate::game::events::InputState;
use crate::game::messages::OutboundMessage;
use crate::game::world::World;

use super::actions::{try_attack, try_gather, try_npc_interaction};

pub fn handle_input(
    world: &mut World,
    config: &Config,
    player_id: PlayerId,
    input: InputState,
    tick: u64,
    delta_seconds: f32,
    outbox: &mut Vec<OutboundMessage>,
) {
    let attack_cooldown_ticks = (config.server.tick_rate * 0.5).ceil() as u64;
    let interact_cooldown_ticks = (config.server.tick_rate * 0.4).ceil() as u64;

    let (do_attack, do_interact) = {
        let Some(player) = world.get_player_mut(&player_id) else {
            return;
        };

        apply_movement(player, config, input.dx, input.dy, delta_seconds);

        let mut attack = false;
        let mut interact = false;

        if input.attack && tick.saturating_sub(player.last_attack_tick) >= attack_cooldown_ticks {
            player.last_attack_tick = tick;
            attack = true;
        }

        if input.interact && tick.saturating_sub(player.last_interact_tick) >= interact_cooldown_ticks {
            player.last_interact_tick = tick;
            interact = true;
        }

        (attack, interact)
    };

    if do_attack {
        if !try_gather(world, config, player_id) {
            try_attack(world, config, player_id);
        }
    }

    if do_interact {
        try_npc_interaction(world, player_id, outbox);
    }
}

fn apply_movement(
    player: &mut crate::game::entities::PlayerState,
    config: &Config,
    dx: f32,
    dy: f32,
    delta_seconds: f32,
) {
    let mut vx = dx;
    let mut vy = dy;
    let len = (vx * vx + vy * vy).sqrt();
    if len > 1.0e-6 {
        vx /= len;
        vy /= len;
    }

    let speed_bonus: f32 = player
        .stat_bonuses
        .iter()
        .filter(|bonus| bonus.stat == "speed")
        .map(|bonus| bonus.value)
        .sum();
    let speed = config.balance.player.base_speed + speed_bonus;
    let move_x = vx * speed * delta_seconds;
    let move_y = vy * speed * delta_seconds;
    if move_x.abs() > 0.0 || move_y.abs() > 0.0 {
        player.x += move_x;
        player.y += move_y;
        player.stats.steps = player
            .stats
            .steps
            .saturating_add(((move_x.abs() + move_y.abs()) * 10.0) as u64);
    }
}
