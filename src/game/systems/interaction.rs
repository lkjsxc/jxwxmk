use uuid::Uuid;

use crate::config::Config;
use crate::game::world::entities::Item;
use crate::game::world::World;
use crate::protocol::client::InputState;
use crate::protocol::server::{NpcInteractionData, ServerMessage};

use super::combat;

pub struct InteractionOutputs {
    pub npc_messages: Vec<(Uuid, ServerMessage)>,
    pub gathered: Vec<(Uuid, String, u32)>,
    pub kills: Vec<(Uuid, String)>,
}

impl InteractionOutputs {
    pub fn new() -> Self {
        Self {
            npc_messages: Vec::new(),
            gathered: Vec::new(),
            kills: Vec::new(),
        }
    }
}

pub fn apply_inputs(
    world: &mut World,
    config: &Config,
    tick_rate: f32,
    inputs: &[(Uuid, InputState)],
) -> InteractionOutputs {
    let mut outputs = InteractionOutputs::new();
    let speed = config.balance.player.base_speed;
    let dt = 1.0 / tick_rate;

    for (player_id, input) in inputs {
        let player = match world.players.get_mut(player_id) {
            Some(player) => player,
            None => continue,
        };
        if !player.spawned {
            continue;
        }
        let dx = input.dx.clamp(-1.0, 1.0);
        let dy = input.dy.clamp(-1.0, 1.0);
        if dx.abs() > f32::EPSILON || dy.abs() > f32::EPSILON {
            player.x += dx * speed * dt;
            player.y += dy * speed * dt;
            player.stats.steps += 1;
        }

        if input.interact {
            if let Some(message) = build_npc_interaction(world, player_id) {
                outputs.npc_messages.push((player_id, message));
            }
        }

        if input.attack {
            if let Some((coord, resource_id)) = world.nearby_resource(player.x, player.y, 60.0) {
                if let Some(chunk) = world.chunks.get_mut(&coord) {
                    if let Some(resource) = chunk.resources.get_mut(&resource_id) {
                        resource.health -= config.balance.tools.base_gather_damage;
                        if resource.health <= 0.0 {
                            resource.amount = resource.amount.saturating_sub(1);
                            resource.health = resource.max_health;
                            player
                                .inventory
                                .add_item(Item::new(resource.r_type.clone(), 1));
                            player.stats.gathers += 1;
                            outputs.gathered.push((*player_id, resource.r_type.clone(), 1));
                            if resource.amount == 0 {
                                chunk.resources.remove(&resource_id);
                            }
                        }
                    }
                }
            } else if let Some((coord, mob_id)) = world.nearby_mob(player.x, player.y, 60.0) {
                if let Some(chunk) = world.chunks.get_mut(&coord) {
                    if let Some(mob) = chunk.mobs.get_mut(&mob_id) {
                        combat::damage_mob(mob, config.balance.combat.base_melee_damage);
                        if mob.health <= 0.0 {
                            let mob_type = mob.m_type.clone();
                            chunk.mobs.remove(&mob_id);
                            player.stats.kills += 1;
                            outputs.kills.push((*player_id, mob_type));
                        }
                    }
                }
            }
        }
    }

    outputs
}

fn build_npc_interaction(world: &World, player_id: &Uuid) -> Option<ServerMessage> {
    let player = world.players.get(player_id)?;
    for chunk in world.chunks.values() {
        for (id, npc) in &chunk.npcs {
            let dx = npc.x - player.x;
            let dy = npc.y - player.y;
            if (dx * dx + dy * dy).sqrt() <= 80.0 {
                let data = NpcInteractionData {
                    npc_id: id.to_string(),
                    name: npc.name.clone(),
                    text: "Need supplies?".to_string(),
                    options: vec!["Browse".to_string(), "Goodbye".to_string()],
                };
                return Some(ServerMessage::NpcInteraction { data });
            }
        }
    }
    None
}
