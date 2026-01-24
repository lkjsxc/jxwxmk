import { InputState } from './InputState';

export class InputManager {
    private keys: Map<string, boolean>;
    private mousePosition: { x: number; y: number };
    private mouseButtons: Map<number, boolean>;
    private currentInputState: InputState;
    private sequence: number = 0;
    
    constructor() {
        this.keys = new Map();
        this.mousePosition = { x: 0, y: 0 };
        this.mouseButtons = new Map();
        this.currentInputState = new InputState();
    }
    
    public initialize(): void {
        window.addEventListener('keydown', this.handleKeyDown.bind(this));
        window.addEventListener('keyup', this.handleKeyUp.bind(this));
        window.addEventListener('mousemove', this.handleMouseMove.bind(this));
        window.addEventListener('mousedown', this.handleMouseDown.bind(this));
        window.addEventListener('mouseup', this.handleMouseUp.bind(this));
        window.addEventListener('contextmenu', this.handleContextMenu.bind(this));
    }
    
    private handleKeyDown(event: KeyboardEvent): void {
        this.keys.set(event.code, true);
        this.updateInputState();
    }
    
    private handleKeyUp(event: KeyboardEvent): void {
        this.keys.set(event.code, false);
        this.updateInputState();
    }
    
    private handleMouseMove(event: MouseEvent): void {
        this.mousePosition.x = event.clientX;
        this.mousePosition.y = event.clientY;
    }
    
    private handleMouseDown(event: MouseEvent): void {
        this.mouseButtons.set(event.button, true);
        this.updateInputState();
    }
    
    private handleMouseUp(event: MouseEvent): void {
        this.mouseButtons.set(event.button, false);
        this.updateInputState();
    }
    
    private handleContextMenu(event: MouseEvent): void {
        // Prevent context menu on right click
        if (this.mouseButtons.get(2)) {
            event.preventDefault();
        }
    }
    
    private updateInputState(): void {
        const movement = { x: 0, y: 0 };
        const actions: string[] = [];
        let sprint = false;
        
        // Movement keys
        if (this.keys.get('KeyW') || this.keys.get('ArrowUp')) {
            movement.y -= 1;
        }
        if (this.keys.get('KeyS') || this.keys.get('ArrowDown')) {
            movement.y += 1;
        }
        if (this.keys.get('KeyA') || this.keys.get('ArrowLeft')) {
            movement.x -= 1;
        }
        if (this.keys.get('KeyD') || this.keys.get('ArrowRight')) {
            movement.x += 1;
        }
        
        // Normalize diagonal movement
        const length = Math.sqrt(movement.x * movement.x + movement.y * movement.y);
        if (length > 0) {
            movement.x /= length;
            movement.y /= length;
        }
        
        // Sprint
        if (this.keys.get('ShiftLeft') || this.keys.get('ShiftRight')) {
            sprint = true;
        }
        
        // Actions
        if (this.mouseButtons.get(0)) { // Left click
            actions.push('attack');
        }
        if (this.mouseButtons.get(2)) { // Right click
            actions.push('interact');
        }
        if (this.keys.get('KeyE')) {
            actions.push('use_item');
        }
        if (this.keys.get('KeyC')) {
            actions.push('craft');
        }
        
        // Update current input state
        this.currentInputState.movement = movement;
        this.currentInputState.actions = actions;
        this.currentInputState.sprint = sprint;
        this.currentInputState.sequence = this.sequence++;
    }
    
    public getInputState(): InputState {
        return { ...this.currentInputState };
    }
    
    public getMousePosition(): { x: number; y: number } {
        return { ...this.mousePosition };
    }
    
    public isKeyPressed(key: string): boolean {
        return this.keys.get(key) || false;
    }
    
    public isMouseButtonPressed(button: number): boolean {
        return this.mouseButtons.get(button) || false;
    }
}