import { Player } from './Player';
import { World } from './World';

export interface CraftingRecipe {
    id: string;
    name: string;
    requirements: [string, number][];
    result: {
        item_type: string;
        quantity: number;
    };
    crafting_time: number;
    tier: number;
}

export class GameState {
    public player: Player | null;
    public world: World;
    public entities: Map<string, any>;
    public serverTick: number;
    public recipes: CraftingRecipe[];
    
    constructor() {
        this.player = null;
        this.world = new World(2000, 2000);
        this.entities = new Map();
        this.serverTick = 0;
        this.recipes = [];
    }
}