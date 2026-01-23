export class Game {
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;
    private player: Player;
    private world: World;
    private isRunning: boolean = false;
    
    public onUpdate: (gameState: GameState) => void;

    constructor(canvas: HTMLCanvasElement) {
        this.canvas = canvas;
        this.ctx = canvas.getContext('2d')!;
        this.player = new Player(0, 0);
        this.world = new World();
    }

    public start(): void {
        this.isRunning = true;
    }

    public update(): void {
        if (!this.isRunning) return;

        this.player.update();
        this.world.update();
        this.updateUI();

        if (this.onUpdate) {
            this.onUpdate(this.getGameState());
        }
    }

    public render(): void {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        
        this.world.render(this.ctx);
        this.player.render(this.ctx);
    }

    public handleServerMessage(data: any): void {
        switch (data.type) {
            case 'game_state':
                this.updateFromServer(data.payload);
                break;
            case 'player_joined':
                this.world.addPlayer(data.payload);
                break;
        }
    }

    private updateFromServer(gameState: any): void {
        this.world.updateFromServer(gameState.world);
    }

    private getGameState(): GameState {
        return {
            playerId: this.player.id,
            position: this.player.getPosition(),
            action: this.player.getCurrentAction()
        };
    }

    private updateUI(): void {
        const healthBar = document.getElementById('healthBar') as HTMLElement;
        const hungerBar = document.getElementById('hungerBar') as HTMLElement;
        const healthValue = document.getElementById('healthValue') as HTMLElement;
        const hungerValue = document.getElementById('hungerValue') as HTMLElement;

        healthBar.style.width = `${this.player.health}%`;
        hungerBar.style.width = `${this.player.hunger}%`;
        healthValue.textContent = this.player.health.toString();
        hungerValue.textContent = this.player.hunger.toString();
    }
}

class Player {
    public id: string = 'player1';
    public x: number;
    public y: number;
    public health: number = 100;
    public hunger: number = 100;
    private velocity: { x: number; y: number } = { x: 0, y: 0 };

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
        this.setupInput();
    }

    public update(): void {
        this.x += this.velocity.x;
        this.y += this.velocity.y;
        this.hunger -= 0.01;
        if (this.hunger < 0) this.hunger = 0;
    }

    public render(ctx: CanvasRenderingContext2D): void {
        ctx.fillStyle = '#00ff00';
        ctx.fillRect(this.x, this.y, 20, 20);
        
        ctx.fillStyle = '#fff';
        ctx.font = '12px Arial';
        ctx.fillText(this.id, this.x, this.y - 5);
    }

    public getPosition(): { x: number; y: number } {
        return { x: this.x, y: this.y };
    }

    public getCurrentAction(): string {
        return 'idle';
    }

    private setupInput(): void {
        document.addEventListener('keydown', (e) => {
            switch (e.key) {
                case 'ArrowUp':
                case 'w':
                    this.velocity.y = -2;
                    break;
                case 'ArrowDown':
                case 's':
                    this.velocity.y = 2;
                    break;
                case 'ArrowLeft':
                case 'a':
                    this.velocity.x = -2;
                    break;
                case 'ArrowRight':
                case 'd':
                    this.velocity.x = 2;
                    break;
            }
        });

        document.addEventListener('keyup', (e) => {
            switch (e.key) {
                case 'ArrowUp':
                case 'w':
                case 'ArrowDown':
                case 's':
                    this.velocity.y = 0;
                    break;
                case 'ArrowLeft':
                case 'a':
                case 'ArrowRight':
                case 'd':
                    this.velocity.x = 0;
                    break;
            }
        });
    }
}

class World {
    private tiles: Tile[][];
    private otherPlayers: Map<string, any> = new Map();

    constructor() {
        this.tiles = this.generateWorld();
    }

    public update(): void {
    }

    public render(ctx: CanvasRenderingContext2D): void {
        for (let y = 0; y < this.tiles.length; y++) {
            for (let x = 0; x < this.tiles[y].length; x++) {
                this.tiles[y][x].render(ctx, x * 20, y * 20);
            }
        }

        this.otherPlayers.forEach((player, id) => {
            ctx.fillStyle = '#ff0000';
            ctx.fillRect(player.x, player.y, 20, 20);
            
            ctx.fillStyle = '#fff';
            ctx.font = '12px Arial';
            ctx.fillText(id, player.x, player.y - 5);
        });
    }

    public addPlayer(player: any): void {
        this.otherPlayers.set(player.id, player);
    }

    public updateFromServer(worldData: any): void {
    }

    private generateWorld(): Tile[][] {
        const width = 60;
        const height = 40;
        const tiles: Tile[][] = [];

        for (let y = 0; y < height; y++) {
            tiles[y] = [];
            for (let x = 0; x < width; x++) {
                const random = Math.random();
                if (random < 0.1) {
                    tiles[y][x] = new TreeTile();
                } else if (random < 0.15) {
                    tiles[y][x] = new StoneTile();
                } else {
                    tiles[y][x] = new GrassTile();
                }
            }
        }

        return tiles;
    }
}

class Tile {
    public render(ctx: CanvasRenderingContext2D, x: number, y: number): void {
    }
}

class GrassTile extends Tile {
    public render(ctx: CanvasRenderingContext2D, x: number, y: number): void {
        ctx.fillStyle = '#228b22';
        ctx.fillRect(x, y, 20, 20);
    }
}

class TreeTile extends Tile {
    public render(ctx: CanvasRenderingContext2D, x: number, y: number): void {
        ctx.fillStyle = '#228b22';
        ctx.fillRect(x, y, 20, 20);
        
        ctx.fillStyle = '#8b4513';
        ctx.fillRect(x + 8, y + 12, 4, 8);
        
        ctx.fillStyle = '#006400';
        ctx.beginPath();
        ctx.arc(x + 10, y + 10, 8, 0, Math.PI * 2);
        ctx.fill();
    }
}

class StoneTile extends Tile {
    public render(ctx: CanvasRenderingContext2D, x: number, y: number): void {
        ctx.fillStyle = '#228b22';
        ctx.fillRect(x, y, 20, 20);
        
        ctx.fillStyle = '#696969';
        ctx.fillRect(x + 6, y + 6, 8, 8);
    }
}

interface GameState {
    playerId: string;
    position: { x: number; y: number };
    action: string;
}