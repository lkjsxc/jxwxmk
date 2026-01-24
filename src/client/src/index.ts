// Main game entry point
import { GameClient } from './game/GameClient';
import { NetworkManager } from './network/NetworkManager';
import { InputManager } from './input/InputManager';
import { Renderer } from './rendering/Renderer';
import { UIManager } from './ui/UIManager';

class GameApp {
    private gameClient: GameClient;
    private networkManager: NetworkManager;
    private inputManager: InputManager;
    private renderer: Renderer;
    private uiManager: UIManager;
    
    constructor() {
        console.log('Initializing game client...');
        
        // Initialize systems
        this.renderer = new Renderer();
        this.inputManager = new InputManager();
        this.networkManager = new NetworkManager('ws://localhost:8081/ws');
        this.uiManager = new UIManager();
        
        // Create game client
        this.gameClient = new GameClient(
            this.networkManager,
            this.inputManager,
            this.renderer,
            this.uiManager
        );
        
        // Start the game
        this.start();
    }
    
    private async start(): Promise<void> {
        try {
            // Initialize all systems
            await this.renderer.initialize();
            this.inputManager.initialize();
            await this.networkManager.connect();
            this.uiManager.initialize();
            
            // Start game loop
            this.gameClient.start();
            
            console.log('Game started successfully!');
        } catch (error) {
            console.error('Failed to start game:', error);
        }
    }
}

// Start the game when DOM is loaded
window.addEventListener('DOMContentLoaded', () => {
    new GameApp();
});