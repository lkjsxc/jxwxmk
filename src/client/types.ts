export enum AppState { InGame, GameOver }

export interface Item {
    kind: string; // "Wood", "Stone", etc.
    amount: number;
    max_stack: number;
    level?: number;
    xp?: number;
}

export interface Inventory {
    slots: (Item | null)[];
    capacity: number;
}

export interface Achievement {
    id: string;
    name: string;
    description: string;
    stat_bonus: [string, number];
    requirement?: { type: string, value: number };
}

export interface PlayerStats {
    steps_taken: number;
    mobs_killed: number;
    items_crafted: number;
    resources_gathered: number;
    structures_placed: number;
    damage_taken: number;
    deaths: number;
}

export type QuestState = "NotStarted" | "InProgress" | "ReadyToTurnIn" | "Completed";

export type ObjectiveType = 
    | { Gather: { item: string, count: number, current: number } }
    | { Kill: { mob_type: string, count: number, current: number } }
    | { TalkTo: { npc_name: string } };

export interface Quest {
    id: string;
    name: string;
    description: string;
    state: QuestState;
    objectives: ObjectiveType[];
}

export interface Player {
    id: string;
    username: string;
    x: number;
    y: number;
    health: number;
    hunger: number;
    cold: number;
    inventory: Inventory;
    active_slot: number;
    lastHitAt?: number;
    achievements: string[];
    quests: Quest[];
    stat_bonuses: Record<string, number>;
    stats?: PlayerStats;
}

export interface Resource {
    id: string;
    r_type: "Tree" | "Rock" | "Food";
    x: number;
    y: number;
    amount: number;
    lastHitAt?: number;
}

export interface Mob {
    id: string;
    m_type: "Rabbit" | "Wolf" | "Bear";
    x: number;
    y: number;
    health: number;
    lastHitAt?: number;
    level?: number;
}

export interface Structure {
    id: string;
    s_type: "Wall" | "Door" | "Torch" | "Workbench";
    x: number;
    y: number;
    health: number;
    owner_id: string;
    lastHitAt?: number;
}

export interface BarrierCore {
    id: string;
    x: number;
    y: number;
    level: number;
    base_range: number;
}

export interface World {
    width: number;
    height: number;
    players: Record<string, Player>;
    resources: Record<string, Resource>;
    mobs: Record<string, Mob>;
    structures: Record<string, Structure>;
    npcs: Record<string, Npc>;
    barrier_cores: Record<string, BarrierCore>;
}

export interface NpcInteraction {
    npc_id: string;
    npc_type: string;
    name: string;
    text: string;
    options: string[];
    trade_items?: Item[];
}

export interface InputState {
    dx: number;
    dy: number;
    attack: boolean;
    interact: boolean;
    craft?: string;
    slot?: number;
    name?: string;
    swapSlots?: { from: number; to: number };
}
