import { Player } from './Player';
import { World } from './World';

export class GameState {
    public player: Player | null;
    public world: World;
    public entities: Map<string, any>;
    public serverTick: number;
    
    constructor() {
        this.player = null;
        this.world = new World(2000, 2000);
        this.entities = new Map();
        this.serverTick = 0;
    }
}