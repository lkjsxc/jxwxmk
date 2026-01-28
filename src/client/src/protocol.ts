export type ClientMessage =
    | { type: 'input', data: InputData }
    | { type: 'spawn', data: { settlement_id: string | null } }
    | { type: 'craft', data: { recipe: string } }
    | { type: 'trade', data: { npc_id: string, item: string, count: number, buy: boolean } }
    | { type: 'npcAction', data: { npc_id: string, option: number } }
    | { type: 'acceptQuest', data: { quest_id: string } }
    | { type: 'slot', data: { slot: number } }
    | { type: 'swapSlots', data: { from: number, to: number } }
    | { type: 'name', data: { name: string } };

export interface InputData {
    dx: number;
    dy: number;
    attack: boolean;
    interact: boolean;
    aim?: { x: number, y: number };
}

export type ServerMessage =
    | { type: 'welcome', id: string, token: string, version: number, spawned: boolean }
    | { type: 'sessionRevoked', reason: string }
    | { type: 'chunkAdd', data: ChunkAddData }
    | { type: 'chunkRemove', data: { coord: [number, number] } }
    | { type: 'entityDelta', data: EntityDeltaData }
    | { type: 'achievement', data: { id: string, name: string } }
    | { type: 'notification', data: { text: string } }
    | { type: 'error', data: ErrorData }
    | { type: 'npcInteraction', data: NpcInteractionData }
    | { type: 'questUpdate', data: { quest: any } }
    | { type: 'playerUpdate', data: PlayerUpdateData };

export interface PlayerUpdateData {
    inventory: (InventorySlot | null)[];
    active_slot: number;
    xp: number;
    level: number;
    stats: Record<string, number>;
    quests: any[];
}

export interface InventorySlot {
    item_id: string;
    count: number;
}

export interface ChunkAddData {
    coord: [number, number];
    biome: string;
    entities: {
        resources: Record<string, EntitySnapshot>;
        mobs: Record<string, EntitySnapshot>;
        structures: Record<string, EntitySnapshot>;
        npcs: Record<string, EntitySnapshot>;
    };
}

export interface EntitySnapshot {
    id: string;
    kind: string;
    subtype: string;
    x: number;
    y: number;
    hp?: number;
    max_hp?: number;
    hunger?: number;
    temp?: number;
    level?: number;
    name?: string | null;
}

export interface EntityDeltaData {
    chunk: [number, number];
    updates: EntitySnapshot[];
    removes: { id: string, kind: string }[];
}

export interface ErrorData {
    code: string;
    message: string;
    details?: any;
}

export interface NpcInteractionData {
    npc_id: string;
    name: string;
    text: string;
    options: string[];
}
