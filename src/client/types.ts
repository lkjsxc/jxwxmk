export interface Inventory {
    wood: number;
    stone: number;
    food: number;
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

export interface World {
    width: number;
    height: number;
    players: Record<string, Player>;
    resources: Record<string, Resource>;
    // mobs: Record<string, Mob>; // To be added
}

export interface InputState {
    dx: number;
    dy: number;
    attack: boolean;
}
