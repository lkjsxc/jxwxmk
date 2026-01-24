export class InputState {
    public movement: { x: number; y: number };
    public actions: string[];
    public sprint: boolean;
    public sequence: number;
    
    constructor() {
        this.movement = { x: 0, y: 0 };
        this.actions = [];
        this.sprint = false;
        this.sequence = 0;
    }
}