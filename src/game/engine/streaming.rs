use crate::protocol::server::ServerMessage;

use super::GameEngine;

impl GameEngine {
    pub(crate) fn build_chunk_add(
        &self,
        coord: crate::game::world::coords::ChunkCoord,
    ) -> Option<ServerMessage> {
        let chunk = self.world.chunks.get(&coord)?;
        let mut entities = crate::protocol::types::ChunkEntities::default();
        for (id, node) in &chunk.resources {
            entities.resources.insert(
                id.to_string(),
                crate::protocol::types::EntityUpdate {
                    id: id.to_string(),
                    kind: crate::protocol::types::EntityKind::Resource,
                    subtype: node.r_type.clone(),
                    x: node.x,
                    y: node.y,
                    hp: Some(node.health),
                    max_hp: Some(node.max_health),
                    level: Some(node.level),
                    name: None,
                    range: None,
                },
            );
        }
        for (id, mob) in &chunk.mobs {
            entities.mobs.insert(
                id.to_string(),
                crate::protocol::types::EntityUpdate {
                    id: id.to_string(),
                    kind: crate::protocol::types::EntityKind::Mob,
                    subtype: mob.m_type.clone(),
                    x: mob.x,
                    y: mob.y,
                    hp: Some(mob.health),
                    max_hp: Some(mob.max_health),
                    level: Some(mob.level),
                    name: None,
                    range: None,
                },
            );
        }
        for (id, structure) in &chunk.structures {
            entities.structures.insert(
                id.to_string(),
                crate::protocol::types::EntityUpdate {
                    id: id.to_string(),
                    kind: crate::protocol::types::EntityKind::Structure,
                    subtype: structure.s_type.clone(),
                    x: structure.x,
                    y: structure.y,
                    hp: Some(structure.health),
                    max_hp: Some(structure.max_health),
                    level: Some(structure.tier),
                    name: None,
                    range: None,
                },
            );
        }
        for (id, npc) in &chunk.npcs {
            entities.npcs.insert(
                id.to_string(),
                crate::protocol::types::EntityUpdate {
                    id: id.to_string(),
                    kind: crate::protocol::types::EntityKind::Npc,
                    subtype: npc.role.clone(),
                    x: npc.x,
                    y: npc.y,
                    hp: None,
                    max_hp: None,
                    level: None,
                    name: Some(npc.name.clone()),
                    range: None,
                },
            );
        }

        Some(ServerMessage::ChunkAdd {
            data: crate::protocol::types::ChunkAddData {
                coord: [coord.x, coord.y],
                biome: chunk.biome_id.clone(),
                entities,
            },
        })
    }

    pub(crate) fn build_entity_delta(
        &self,
        coord: crate::game::world::coords::ChunkCoord,
    ) -> Option<ServerMessage> {
        let chunk = self.world.chunks.get(&coord)?;
        let mut updates = Vec::new();
        let removes = Vec::new();

        for player in self.world.players.values() {
            if player.chunk_x == coord.x && player.chunk_y == coord.y {
                updates.push(crate::protocol::types::EntityUpdate {
                    id: player.id.to_string(),
                    kind: crate::protocol::types::EntityKind::Player,
                    subtype: "player".to_string(),
                    x: player.x,
                    y: player.y,
                    hp: Some(player.health),
                    max_hp: Some(self.config.balance.player.max_health),
                    level: Some(player.level as u8),
                    name: Some(player.username.clone()),
                    range: None,
                });
            }
        }

        for (id, node) in &chunk.resources {
            updates.push(crate::protocol::types::EntityUpdate {
                id: id.to_string(),
                kind: crate::protocol::types::EntityKind::Resource,
                subtype: node.r_type.clone(),
                x: node.x,
                y: node.y,
                hp: Some(node.health),
                max_hp: Some(node.max_health),
                level: Some(node.level),
                name: None,
                range: None,
            });
        }
        for (id, mob) in &chunk.mobs {
            updates.push(crate::protocol::types::EntityUpdate {
                id: id.to_string(),
                kind: crate::protocol::types::EntityKind::Mob,
                subtype: mob.m_type.clone(),
                x: mob.x,
                y: mob.y,
                hp: Some(mob.health),
                max_hp: Some(mob.max_health),
                level: Some(mob.level),
                name: None,
                range: None,
            });
        }
        for (id, structure) in &chunk.structures {
            updates.push(crate::protocol::types::EntityUpdate {
                id: id.to_string(),
                kind: crate::protocol::types::EntityKind::Structure,
                subtype: structure.s_type.clone(),
                x: structure.x,
                y: structure.y,
                hp: Some(structure.health),
                max_hp: Some(structure.max_health),
                level: Some(structure.tier),
                name: None,
                range: if structure.s_type == "BarrierCore" {
                    Some(self.config.settlements.core_base_range)
                } else {
                    None
                },
            });
        }
        for (id, npc) in &chunk.npcs {
            updates.push(crate::protocol::types::EntityUpdate {
                id: id.to_string(),
                kind: crate::protocol::types::EntityKind::Npc,
                subtype: npc.role.clone(),
                x: npc.x,
                y: npc.y,
                hp: None,
                max_hp: None,
                level: None,
                name: Some(npc.name.clone()),
                range: None,
            });
        }

        Some(ServerMessage::EntityDelta {
            data: crate::protocol::types::EntityDeltaData {
                chunk: [coord.x, coord.y],
                updates,
                removes,
            },
        })
    }
}
