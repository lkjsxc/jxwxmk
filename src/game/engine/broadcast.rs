use super::GameEngine;

impl GameEngine {
    pub(crate) fn broadcast_deltas(&mut self) {
        let view_radius = self.config.world.view_radius;
        for player in self.world.players.values() {
            if !player.spawned {
                continue;
            }
            let player_id = player.id;
            let new_interest = self.world.interest_set(player, view_radius);
            let old_interest = self.world.interest_sets.entry(player_id).or_default();

            let added: Vec<_> = new_interest.difference(old_interest).cloned().collect();
            let removed: Vec<_> = old_interest.difference(&new_interest).cloned().collect();
            *old_interest = new_interest;

            for coord in added {
                self.world.ensure_chunk(coord, &self.config);
                if let Some(message) = self.build_chunk_add(coord) {
                    self.send_to_player(player_id, message);
                }
            }
            for coord in removed {
                self.send_to_player(
                    player_id,
                    crate::protocol::server::ServerMessage::ChunkRemove {
                        data: crate::protocol::types::ChunkRemoveData {
                            coord: [coord.x, coord.y],
                        },
                    },
                );
            }

            if let Some(interest) = self.world.interest_sets.get(&player_id) {
                for coord in interest {
                    self.world.ensure_chunk(*coord, &self.config);
                    if let Some(message) = self.build_entity_delta(*coord) {
                        self.send_to_player(player_id, message);
                    }
                }
            }
        }
    }
}
