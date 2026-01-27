use crate::protocol::{ChunkAddData, ChunkEntities, EntitySnapshot};

use crate::game::chunk::Chunk;
use crate::game::entities::{BarrierCore, Mob, Npc, PlayerState, ResourceNode, Structure};

pub fn build_chunk_add(chunk: &Chunk) -> ChunkAddData {
    let mut entities = ChunkEntities::default();

    for (id, resource) in &chunk.resources {
        entities
            .resources
            .insert(id.clone(), snapshot_resource(resource));
    }
    for (id, mob) in &chunk.mobs {
        entities.mobs.insert(id.clone(), snapshot_mob(mob));
    }
    for (id, structure) in &chunk.structures {
        entities
            .structures
            .insert(id.clone(), snapshot_structure(structure));
    }
    for (id, npc) in &chunk.npcs {
        entities.npcs.insert(id.clone(), snapshot_npc(npc));
    }

    ChunkAddData {
        coord: [chunk.coord.cx, chunk.coord.cy],
        biome: chunk.biome_id.clone(),
        entities,
    }
}

pub fn snapshot_resource(resource: &ResourceNode) -> EntitySnapshot {
    EntitySnapshot {
        id: resource.id.clone(),
        kind: "resource".to_string(),
        subtype: resource.r_type.clone(),
        x: resource.x,
        y: resource.y,
        hp: Some(resource.amount),
        max_hp: Some(resource.amount.max(1.0)),
        level: Some(resource.level),
        name: None,
        range: None,
    }
}

pub fn snapshot_mob(mob: &Mob) -> EntitySnapshot {
    EntitySnapshot {
        id: mob.id.clone(),
        kind: "mob".to_string(),
        subtype: mob.m_type.clone(),
        x: mob.x,
        y: mob.y,
        hp: Some(mob.health),
        max_hp: Some(mob.max_health),
        level: Some(mob.level),
        name: None,
        range: None,
    }
}

pub fn snapshot_structure(structure: &Structure) -> EntitySnapshot {
    EntitySnapshot {
        id: structure.id.clone(),
        kind: "structure".to_string(),
        subtype: structure.s_type.clone(),
        x: structure.x,
        y: structure.y,
        hp: Some(structure.health),
        max_hp: Some(structure.max_health),
        level: Some(structure.tier),
        name: None,
        range: None,
    }
}

pub fn snapshot_npc(npc: &Npc) -> EntitySnapshot {
    EntitySnapshot {
        id: npc.id.clone(),
        kind: "npc".to_string(),
        subtype: npc.role.clone(),
        x: npc.x,
        y: npc.y,
        hp: None,
        max_hp: None,
        level: None,
        name: Some(npc.name.clone()),
        range: None,
    }
}

pub fn snapshot_player(player: &PlayerState) -> EntitySnapshot {
    EntitySnapshot {
        id: player.id.to_string(),
        kind: "player".to_string(),
        subtype: "player".to_string(),
        x: player.x,
        y: player.y,
        hp: Some(player.health),
        max_hp: Some(player.health.max(1.0)),
        level: Some(player.level),
        name: Some(player.username.clone()),
        range: None,
    }
}

pub fn snapshot_barrier(core: &BarrierCore) -> EntitySnapshot {
    EntitySnapshot {
        id: core.id.clone(),
        kind: "structure".to_string(),
        subtype: "barrier_core".to_string(),
        x: core.x,
        y: core.y,
        hp: Some(core.integrity),
        max_hp: Some(core.integrity.max(1.0)),
        level: Some(core.level),
        name: None,
        range: Some(core.range()),
    }
}
