export class InputState {
    public movement: { x: number; y: number };
    public actions: string[];
    public sprint: boolean;
    
    constructor() {
        this.movement = { x: 0, y: 0 };
        this.actions = [];
        this.sprint = false;
    }
}