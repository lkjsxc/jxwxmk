export interface Item {
    kind: string; // "Wood", "Stone", etc.
    amount: number;
    max_stack: number;
}

export interface Inventory {
    slots: (Item | null)[];
    capacity: number;
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
}

export interface Resource {
    id: string;
    r_type: "Tree" | "Rock" | "Food";
    x: number;
    y: number;
    amount: number;
}

export interface Mob {
    id: string;
    m_type: "Rabbit" | "Wolf" | "Bear";
    x: number;
    y: number;
    health: number;
}

export interface Structure {
    id: string;
    s_type: "Wall" | "Door" | "Torch" | "Workbench";
    x: number;
    y: number;
    health: number;
    owner_id: string;
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
}
