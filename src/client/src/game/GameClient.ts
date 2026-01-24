import { NetworkManager } from '../network/NetworkManager';
import { InputManager } from '../input/InputManager';
import { Renderer } from '../rendering/Renderer';
import { UIManager } from '../ui/UIManager';
import { GameState, CraftingRecipe } from './GameState';
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
        
        this.setupEventListeners();
    }
    
    private setupEventListeners(): void {
        this.networkManager.on('authenticated', (playerId: string) => {
            console.log('Authenticated as player:', playerId);
            this.gameState.player = new Player(playerId);
            this.uiManager.showGameUI();
        });
        
        this.networkManager.on('stateUpdate', (state: any) => {
            this.handleStateUpdate(state);
        });

        this.networkManager.on('recipes', (recipes: CraftingRecipe[]) => {
            this.gameState.recipes = recipes;
            this.uiManager.setRecipes(recipes);
        });
        
        this.networkManager.on('error', (error: string) => {
            console.error('Network error:', error);
            this.uiManager.showError(error);
        });
        
        this.networkManager.on('disconnected', () => {
            console.log('Disconnected from server');
            this.uiManager.showDisconnected();
        });

        this.uiManager.onCraftRequest = (recipeId: string) => {
            this.networkManager.sendCraftRequest(recipeId);
        };
    }
    
    public start(): void {
        if (this.running) return;
        this.running = true;
        this.lastUpdateTime = performance.now();
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
        const deltaTime = (now - this.lastUpdateTime) / 1000;
        this.lastUpdateTime = now;
        this.update(deltaTime);
        this.render();
        requestAnimationFrame(() => this.gameLoop());
    }
    
    private update(deltaTime: number): void {
        const inputState = this.inputManager.getInputState();
        
        if (this.gameState.player) {
            this.networkManager.sendPlayerInput(inputState);
            this.gameState.player.update(deltaTime, inputState);
        }
        
        this.gameState.world.update(deltaTime);
    }
    
    private render(): void {
        this.renderer.renderWorld(this.gameState.world);
        if (this.gameState.player) {
            this.renderer.renderPlayer(this.gameState.player);
        }
        this.uiManager.render(this.gameState);
    }
    
    private handleStateUpdate(state: any): void {
        if (state.entities) {
            state.entities.forEach((entityData: any) => {
                if (this.gameState.player && entityData.id === this.gameState.player.id) {
                    this.gameState.player.reconcile(entityData);
                } else {
                    this.gameState.entities.set(entityData.id, entityData);
                }
            });
            
            for (const id of this.gameState.entities.keys()) {
                if (!state.entities.find((e: any) => e.id === id)) {
                    this.gameState.entities.delete(id);
                }
            }
        }
        
        if (state.tick) {
            this.gameState.serverTick = state.tick;
        }
    }
}