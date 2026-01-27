use actix::prelude::*;

use super::{messages::*, GameEngine};
use super::tick::EngineEvent;
use crate::game::events::{
    AcceptQuestEvent, CraftEvent, InputEvent, NameEvent, NpcActionEvent, SlotEvent, SpawnEvent,
    SwapSlotsEvent, TradeEvent,
};

impl Handler<InputMsg> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: InputMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.enqueue(EngineEvent::Input(InputEvent {
            player_id: msg.player_id,
            input: msg.input,
        }));
    }
}

impl Handler<SpawnMsg> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: SpawnMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.enqueue(EngineEvent::Spawn(SpawnEvent {
            player_id: msg.player_id,
            request: msg.request,
        }));
    }
}

impl Handler<CraftMsg> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: CraftMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.enqueue(EngineEvent::Craft(CraftEvent {
            player_id: msg.player_id,
            request: msg.request,
        }));
    }
}

impl Handler<TradeMsg> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: TradeMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.enqueue(EngineEvent::Trade(TradeEvent {
            player_id: msg.player_id,
            request: msg.request,
        }));
    }
}

impl Handler<NpcActionMsg> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: NpcActionMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.enqueue(EngineEvent::NpcAction(NpcActionEvent {
            player_id: msg.player_id,
            request: msg.request,
        }));
    }
}

impl Handler<AcceptQuestMsg> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: AcceptQuestMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.enqueue(EngineEvent::AcceptQuest(AcceptQuestEvent {
            player_id: msg.player_id,
            request: msg.request,
        }));
    }
}

impl Handler<SlotMsg> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: SlotMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.enqueue(EngineEvent::Slot(SlotEvent {
            player_id: msg.player_id,
            request: msg.request,
        }));
    }
}

impl Handler<SwapSlotsMsg> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: SwapSlotsMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.enqueue(EngineEvent::SwapSlots(SwapSlotsEvent {
            player_id: msg.player_id,
            request: msg.request,
        }));
    }
}

impl Handler<NameMsg> for GameEngine {
    type Result = ();

    fn handle(&mut self, msg: NameMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.enqueue(EngineEvent::Name(NameEvent {
            player_id: msg.player_id,
            request: msg.request,
        }));
    }
}
