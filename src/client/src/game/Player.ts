import { InputState } from '../input/InputState';

interface StateSnapshot {
    position: { x: number; y: number };
    sequence: number;
}

export class Player {
    public id: string;
    public position: { x: number; y: number };
    public velocity: { x: number; y: number };
    public health: number;
    public maxHealth: number;
    public hunger: number;
    public maxHunger: number;
    public thirst: number;
    public maxThirst: number;
    public inventory: Map<string, number>;
    public equipment: Map<string, string>;
    
    private inputBuffer: InputState[] = [];
    private lastAckSequence: number = -1;
    
    constructor(id: string) {
        this.id = id;
        this.position = { x: 0, y: 0 };
        this.velocity = { x: 0, y: 0 };
        this.health = 100;
        this.maxHealth = 100;
        this.hunger = 100;
        this.maxHunger = 100;
        this.thirst = 100;
        this.maxThirst = 100;
        this.inventory = new Map();
        this.equipment = new Map();
    }
    
    public update(deltaTime: number, input: InputState): void {
        // Store input for reconciliation
        this.inputBuffer.push({ ...input });
        
        // Apply input locally
        this.applyInput(deltaTime, input);
    }

    private applyInput(deltaTime: number, input: InputState): void {
        const speed = input.sprint ? 5.0 : 3.0;
        
        this.velocity.x = input.movement.x * speed;
        this.velocity.y = input.movement.y * speed;
        
        this.position.x += this.velocity.x * deltaTime;
        this.position.y += this.velocity.y * deltaTime;
        
        // Survival meters updates (approximate prediction)
        this.hunger = Math.max(0, this.hunger - 0.1 * deltaTime);
        this.thirst = Math.max(0, this.thirst - 0.05 * deltaTime);
    }
    
    public reconcile(serverState: any): void {
        // Update acknowledged sequence
        if (serverState.last_sequence !== undefined) {
            this.lastAckSequence = serverState.last_sequence;
        }

        // Apply server state as ground truth
        if (serverState.position) {
            this.position = { ...serverState.position };
        }
        
        if (serverState.health !== undefined) this.health = serverState.health;
        if (serverState.hunger !== undefined) this.hunger = serverState.hunger;
        if (serverState.thirst !== undefined) this.thirst = serverState.thirst;

        // Discard inputs acknowledged by server
        this.inputBuffer = this.inputBuffer.filter(input => input.sequence > this.lastAckSequence);

        // Replay unacknowledged inputs
        const tempDeltaTime = 1.0 / 60.0; // Assume 60fps for re-simulation or store delta in buffer
        this.inputBuffer.forEach(input => {
            this.applyInput(tempDeltaTime, input);
        });
    }
    
    public addToInventory(itemType: string, quantity: number): void {
        const current = this.inventory.get(itemType) || 0;
        this.inventory.set(itemType, current + quantity);
    }
    
    public removeFromInventory(itemType: string, quantity: number): boolean {
        const current = this.inventory.get(itemType) || 0;
        if (current < quantity) return false;
        this.inventory.set(itemType, current - quantity);
        return true;
    }
}