import { NetworkManager } from '../network/NetworkManager';
import { InputManager } from '../input/InputManager';
import { Renderer } from '../rendering/Renderer';
import { UIManager } from '../ui/UIManager';
import { GameState } from './GameState';
import { Player } from './Player';
import { World } from './World';

export class GameClient {
    private networkManager: NetworkManager;
    private inputManager: InputManager;
    private renderer: Renderer;
    private uiManager: UIManager;
    private gameState: GameState;
    private lastUpdateTime: number;
    private running: boolean;
    
    constructor(
        networkManager: NetworkManager,
        inputManager: InputManager,
        renderer: Renderer,
        uiManager: UIManager
    ) {
        this.networkManager = networkManager;
        this.inputManager = inputManager;
        this.renderer = renderer;
        this.uiManager = uiManager;
        this.gameState = new GameState();
        this.lastUpdateTime = 0;
        this.running = false;
        
        // Set up event listeners
        this.setupEventListeners();
    }
    
    private setupEventListeners(): void {
        // Network events
        this.networkManager.on('authenticated', (playerId: string) => {
            console.log('Authenticated as player:', playerId);
            this.gameState.player = new Player(playerId);
            this.uiManager.showGameUI();
        });
        
        this.networkManager.on('stateUpdate', (state: any) => {
            this.handleStateUpdate(state);
        });
        
        this.networkManager.on('error', (error: string) => {
            console.error('Network error:', error);
            this.uiManager.showError(error);
        });
        
        this.networkManager.on('disconnected', () => {
            console.log('Disconnected from server');
            this.uiManager.showDisconnected();
        });
    }
    
    public start(): void {
        if (this.running) return;
        
        this.running = true;
        this.lastUpdateTime = performance.now();
        
        // Start game loop
        requestAnimationFrame(() => this.gameLoop());
        
        console.log('Game loop started');
    }
    
    public stop(): void {
        this.running = false;
        console.log('Game loop stopped');
    }
    
    private gameLoop(): void {
        if (!this.running) return;
        
        const now = performance.now();
        const deltaTime = (now - this.lastUpdateTime) / 1000; // Convert to seconds
        this.lastUpdateTime = now;
        
        // Update game state
        this.update(deltaTime);
        
        // Render
        this.render();
        
        // Continue loop
        requestAnimationFrame(() => this.gameLoop());
    }
    
    private update(deltaTime: number): void {
        // Update input
        const inputState = this.inputManager.getInputState();
        
        // Send input to server if authenticated
        if (this.gameState.player) {
            this.networkManager.sendPlayerInput(inputState);
        }
        
        // Update local game state prediction
        if (this.gameState.player) {
            this.gameState.player.update(deltaTime, inputState);
        }
        
        // Update world
        this.gameState.world.update(deltaTime);
    }
    
    private render(): void {
        // Render world
        this.renderer.renderWorld(this.gameState.world);
        
        // Render entities
        if (this.gameState.player) {
            this.renderer.renderPlayer(this.gameState.player);
        }
        
        // Render UI
        this.uiManager.render(this.gameState);
    }
    
    private handleStateUpdate(state: any): void {
        // Update game state from server
        if (state.entities) {
            // Update entity positions and states
        }
        
        if (state.world) {
            // Update world state
        }
        
        if (state.player) {
            // Update player state and reconcile with local prediction
            if (this.gameState.player) {
                this.gameState.player.reconcile(state.player);
            }
        }
    }
}