use std::collections::HashMap;
use uuid::Uuid;

use crate::game::events::{
    AcceptQuestEvent, CraftEvent, InputEvent, NameEvent, NpcActionEvent, SlotEvent, SpawnEvent,
    SwapSlotsEvent, TradeEvent,
};
use crate::game::systems::{
    achievements, barriers, crafting, death, interaction, quests, survival,
};
use crate::protocol::client::InputState;

use super::GameEngine;

pub(crate) enum EngineEvent {
    Input(InputEvent),
    Spawn(SpawnEvent),
    Craft(CraftEvent),
    Trade(TradeEvent),
    NpcAction(NpcActionEvent),
    AcceptQuest(AcceptQuestEvent),
    Slot(SlotEvent),
    SwapSlots(SwapSlotsEvent),
    Name(NameEvent),
}

impl GameEngine {
    pub(crate) fn tick(&mut self) {
        self.tick_counter = self.tick_counter.wrapping_add(1);
        let mut inputs: HashMap<Uuid, InputState> = HashMap::new();
        while let Some(event) = self.queue.pop_front() {
            match event {
                EngineEvent::Input(event) => {
                    inputs.insert(event.player_id, event.input);
                }
                EngineEvent::Spawn(event) => self.handle_spawn(event),
                EngineEvent::Craft(event) => self.handle_craft(event),
                EngineEvent::Trade(_) => {}
                EngineEvent::NpcAction(_) => {}
                EngineEvent::AcceptQuest(event) => self.handle_accept(event),
                EngineEvent::Slot(event) => self.handle_slot(event),
                EngineEvent::SwapSlots(event) => self.handle_swap_slots(event),
                EngineEvent::Name(event) => self.handle_name(event),
            }
        }

        let mut input_list: Vec<(Uuid, InputState)> = inputs.into_iter().collect();
        input_list.sort_by_key(|(id, _)| *id.as_bytes());
        let outputs = interaction::apply_inputs(
            &mut self.world,
            &self.config,
            self.config.server.tick_rate,
            &input_list,
        );
        let player_ids: Vec<_> = self.world.players.keys().cloned().collect();
        for player_id in player_ids {
            self.world
                .update_player_chunk(player_id, self.config.world.chunk_size);
        }

        for (player_id, message) in outputs.npc_messages {
            self.send_to_player(player_id, message);
        }
        for (player_id, item, count) in outputs.gathered {
            if let Some(player) = self.world.players.get_mut(&player_id) {
                let messages = quests::apply_gather(player, &item, count);
                for message in messages {
                    self.send_to_player(player_id, message);
                }
            }
        }
        for (player_id, mob) in outputs.kills {
            if let Some(player) = self.world.players.get_mut(&player_id) {
                let messages = quests::apply_kill(player, &mob, 1);
                for message in messages {
                    self.send_to_player(player_id, message);
                }
            }
        }

        survival::tick(&mut self.world, &self.config, self.config.server.tick_rate);
        death::handle_deaths(&mut self.world, &self.config);
        barriers::enforce(&mut self.world, &self.config);

        for (player_id, message) in achievements::evaluate(&mut self.world, &self.config) {
            self.send_to_player(player_id, message);
        }

        self.persist_players_if_needed();
        self.broadcast_deltas();
    }

    fn handle_spawn(&mut self, event: SpawnEvent) {
        let mut bound_settlement = None;
        if let Some(player) = self.world.players.get_mut(&event.player_id) {
            if let Some(settlement_id) = event.request.settlement_id {
                if let Ok(uuid) = Uuid::parse_str(&settlement_id) {
                    player.bound_settlement = Some(uuid);
                }
            }
            bound_settlement = player.bound_settlement;
        }

        if let Some((spawn_x, spawn_y)) = death::select_spawn(&self.world, bound_settlement) {
            if let Some(player) = self.world.players.get_mut(&event.player_id) {
                death::apply_respawn(player, spawn_x, spawn_y);
            }
        }
    }

    fn handle_craft(&mut self, event: CraftEvent) {
        if let Some(player) = self.world.players.get_mut(&event.player_id) {
            if crafting::craft(player, &self.config, &event.request.recipe) {
                let messages = quests::apply_craft(player, &event.request.recipe, 1);
                for message in messages {
                    self.send_to_player(event.player_id, message);
                }
            }
        }
    }

    fn handle_accept(&mut self, event: AcceptQuestEvent) {
        if let Some(player) = self.world.players.get_mut(&event.player_id) {
            if let Some(message) = quests::accept(player, &self.config, &event.request.quest_id) {
                self.send_to_player(event.player_id, message);
            }
        }
    }

    fn handle_slot(&mut self, event: SlotEvent) {
        if let Some(player) = self.world.players.get_mut(&event.player_id) {
            if event.request.slot < player.inventory.slots.len() {
                player.inventory.active_slot = event.request.slot;
            }
        }
    }

    fn handle_swap_slots(&mut self, event: SwapSlotsEvent) {
        if let Some(player) = self.world.players.get_mut(&event.player_id) {
            player.inventory.swap_slots(event.request.from, event.request.to);
        }
    }

    fn handle_name(&mut self, event: NameEvent) {
        if let Some(player) = self.world.players.get_mut(&event.player_id) {
            let name = event.request.name.trim();
            if !name.is_empty() && name.len() <= 24 {
                player.username = name.to_string();
            }
        }
    }
}
