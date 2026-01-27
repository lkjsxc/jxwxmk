use super::GameEngine;

impl GameEngine {
    pub(crate) fn broadcast_deltas(&mut self) {
        let view_radius = self.config.world.view_radius;
        let players: Vec<_> = self
            .world
            .players
            .values()
            .filter(|player| player.spawned)
            .map(|player| (player.id, player.chunk_x, player.chunk_y))
            .collect();

        for (player_id, chunk_x, chunk_y) in players {
            let new_interest = crate::game::world::World::interest_set_coords(
                chunk_x,
                chunk_y,
                view_radius,
            );
            let (added, removed, interest) = {
                let old_interest = self.world.interest_sets.entry(player_id).or_default();
                let added: Vec<_> = new_interest.difference(old_interest).cloned().collect();
                let removed: Vec<_> = old_interest.difference(&new_interest).cloned().collect();
                *old_interest = new_interest;
                let interest: Vec<_> = old_interest.iter().cloned().collect();
                (added, removed, interest)
            };

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

            for coord in interest {
                self.world.ensure_chunk(coord, &self.config);
                if let Some(message) = self.build_entity_delta(coord) {
                    self.send_to_player(player_id, message);
                }
            }
        }
    }
}
