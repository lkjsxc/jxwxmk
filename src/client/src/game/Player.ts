import { InputState } from '../input/InputState';

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
        // Update movement based on input
        const speed = input.sprint ? 5.0 : 3.0;
        
        this.velocity.x = input.movement.x * speed;
        this.velocity.y = input.movement.y * speed;
        
        // Apply velocity to position
        this.position.x += this.velocity.x * deltaTime;
        this.position.y += this.velocity.y * deltaTime;
        
        // Update survival meters (simplified)
        this.hunger = Math.max(0, this.hunger - 0.1 * deltaTime);
        this.thirst = Math.max(0, this.thirst - 0.05 * deltaTime);
        
        // Health regeneration based on hunger/thirst
        if (this.hunger > 70 && this.thirst > 70) {
            this.health = Math.min(this.maxHealth, this.health + 0.2 * deltaTime);
        } else if (this.hunger < 30 || this.thirst < 30) {
            this.health = Math.max(0, this.health - 0.3 * deltaTime);
        }
    }
    
    public reconcile(serverState: any): void {
        // Reconcile client prediction with server state
        if (serverState.position) {
            this.position = serverState.position;
        }
        
        if (serverState.health !== undefined) {
            this.health = serverState.health;
        }
        
        if (serverState.hunger !== undefined) {
            this.hunger = serverState.hunger;
        }
        
        if (serverState.thirst !== undefined) {
            this.thirst = serverState.thirst;
        }
    }
    
    public addToInventory(itemType: string, quantity: number): void {
        const current = this.inventory.get(itemType) || 0;
        this.inventory.set(itemType, current + quantity);
    }
    
    public removeFromInventory(itemType: string, quantity: number): boolean {
        const current = this.inventory.get(itemType) || 0;
        if (current < quantity) {
            return false;
        }
        this.inventory.set(itemType, current - quantity);
        return true;
    }
    
    public equipItem(slot: string, itemId: string): void {
        this.equipment.set(slot, itemId);
    }
    
    public unequipItem(slot: string): void {
        this.equipment.delete(slot);
    }
}