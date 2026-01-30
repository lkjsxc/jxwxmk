use protocol::*;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum GameEvent {
    Join { player_id: Uuid, name: String },
    Leave { player_id: Uuid },
    Input { player_id: Uuid, data: InputData },
    Spawn { player_id: Uuid, data: SpawnData },
    Craft { player_id: Uuid, data: CraftData },
    Trade { player_id: Uuid, data: TradeData },
    NpcAction { player_id: Uuid, data: NpcActionData },
    AcceptQuest { player_id: Uuid, data: AcceptQuestData },
    Slot { player_id: Uuid, data: SlotData },
    SwapSlots { player_id: Uuid, data: SwapSlotsData },
    Name { player_id: Uuid, data: NameData },
}

#[derive(Debug, Clone)]
pub enum GameResponse {
    Welcome { player_id: Uuid, message: ServerMessage },
    PlayerUpdate { player_id: Uuid, message: ServerMessage },
    Broadcast { message: ServerMessage },
    ToPlayer { player_id: Uuid, message: ServerMessage },
    Error { player_id: Uuid, error: ErrorData },
}
