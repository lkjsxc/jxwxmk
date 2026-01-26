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

export interface World {
    width: number;
    height: number;
    players: Record<string, Player>;
    resources: Record<string, Resource>;
    mobs: Record<string, Mob>;
    structures: Record<string, Structure>;
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
