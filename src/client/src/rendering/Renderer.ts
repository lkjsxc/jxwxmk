import { Player } from '../game/Player';
import { World } from '../game/World';

export class Renderer {
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;
    private camera: Camera;
    
    constructor() {
        const canvas = document.getElementById('game-canvas') as HTMLCanvasElement;
        if (!canvas) {
            throw new Error('Canvas element not found');
        }
        
        this.canvas = canvas;
        const ctx = canvas.getContext('2d');
        if (!ctx) {
            throw new Error('Could not get 2D rendering context');
        }
        
        this.ctx = ctx;
        this.camera = new Camera();
    }
    
    public initialize(): void {
        // Set canvas size to match window
        this.resizeCanvas();
        window.addEventListener('resize', this.resizeCanvas.bind(this));
        
        // Initial clear
        this.clearScreen();
    }
    
    private resizeCanvas(): void {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
        
        // Update camera
        this.camera.width = this.canvas.width;
        this.camera.height = this.canvas.height;
    }
    
    private clearScreen(): void {
        this.ctx.fillStyle = '#222';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
    }
    
    public renderWorld(world: World): void {
        this.clearScreen();
        
        // Save context state
        this.ctx.save();
        
        // Apply camera transform
        this.ctx.translate(-this.camera.x, -this.camera.y);
        
        // Draw world background
        this.drawWorldBackground(world);
        
        // Draw resources
        this.drawResources(world.resources);
        
        // Restore context state
        this.ctx.restore();
    }
    
    private drawWorldBackground(world: World): void {
        // Draw a simple grid pattern for the world
        const gridSize = 50;
        const gridColor = '#333';
        
        this.ctx.strokeStyle = gridColor;
        this.ctx.lineWidth = 1;
        
        // Draw vertical lines
        for (let x = this.camera.x % gridSize; x < this.camera.x + this.camera.width; x += gridSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(x, this.camera.y);
            this.ctx.lineTo(x, this.camera.y + this.camera.height);
            this.ctx.stroke();
        }
        
        // Draw horizontal lines
        for (let y = this.camera.y % gridSize; y < this.camera.y + this.camera.height; y += gridSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(this.camera.x, y);
            this.ctx.lineTo(this.camera.x + this.camera.width, y);
            this.ctx.stroke();
        }
    }
    
    private drawResources(resources: any[]): void {
        resources.forEach(resource => {
            const screenX = resource.position.x - this.camera.x;
            const screenY = resource.position.y - this.camera.y;
            
            // Only draw resources that are on screen
            if (screenX < -50 || screenX > this.camera.width + 50 ||
                screenY < -50 || screenY > this.camera.height + 50) {
                return;
            }
            
            // Draw different resource types
            switch (resource.type) {
                case 'tree':
                    this.drawTree(screenX, screenY, resource.quantity / resource.maxQuantity);
                    break;
                case 'rock':
                    this.drawRock(screenX, screenY, resource.quantity / resource.maxQuantity);
                    break;
                case 'bush':
                    this.drawBush(screenX, screenY, resource.quantity / resource.maxQuantity);
                    break;
                case 'water':
                    this.drawWater(screenX, screenY);
                    break;
                default:
                    this.drawGenericResource(screenX, screenY);
            }
        });
    }
    
    private drawTree(x: number, y: number, health: number): void {
        // Draw tree trunk
        this.ctx.fillStyle = '#8B4513';
        this.ctx.fillRect(x - 5, y - 20, 10, 30);
        
        // Draw tree leaves (color changes based on health)
        const greenValue = Math.floor(100 + 155 * health);
        this.ctx.fillStyle = `rgb(34, ${greenValue}, 20)`;
        this.ctx.beginPath();
        this.ctx.ellipse(x, y - 30, 20, 25, 0, 0, Math.PI * 2);
        this.ctx.fill();
    }
    
    private drawRock(x: number, y: number, health: number): void {
        // Draw rock (color changes based on health)
        const grayValue = Math.floor(100 + 155 * health);
        this.ctx.fillStyle = `rgb(${grayValue}, ${grayValue}, ${grayValue})`;
        this.ctx.beginPath();
        this.ctx.ellipse(x, y, 15, 12, 0, 0, Math.PI * 2);
        this.ctx.fill();
    }
    
    private drawBush(x: number, y: number, health: number): void {
        // Draw bush (color changes based on health)
        const greenValue = Math.floor(50 + 205 * health);
        this.ctx.fillStyle = `rgb(34, ${greenValue}, 20)`;
        this.ctx.beginPath();
        this.ctx.ellipse(x, y, 12, 8, 0, 0, Math.PI * 2);
        this.ctx.fill();
    }
    
    private drawWater(x: number, y: number): void {
        this.ctx.fillStyle = '#4682B4';
        this.ctx.beginPath();
        this.ctx.rect(x - 20, y - 20, 40, 40);
        this.ctx.fill();
    }
    
    private drawGenericResource(x: number, y: number): void {
        this.ctx.fillStyle = '#FFD700';
        this.ctx.beginPath();
        this.ctx.arc(x, y, 10, 0, Math.PI * 2);
        this.ctx.fill();
    }
    
    public renderPlayer(player: Player): void {
        // Save context state
        this.ctx.save();
        
        // Apply camera transform
        this.ctx.translate(-this.camera.x, -this.camera.y);
        
        // Draw player
        const screenX = player.position.x;
        const screenY = player.position.y;
        
        // Draw player body
        this.ctx.fillStyle = '#FF6B6B';
        this.ctx.beginPath();
        this.ctx.arc(screenX, screenY, 15, 0, Math.PI * 2);
        this.ctx.fill();
        
        // Draw player direction indicator
        const angle = Math.atan2(player.velocity.y, player.velocity.x);
        if (player.velocity.x !== 0 || player.velocity.y !== 0) {
            this.ctx.strokeStyle = '#FFFFFF';
            this.ctx.lineWidth = 2;
            this.ctx.beginPath();
            this.ctx.moveTo(screenX, screenY);
            this.ctx.lineTo(
                screenX + Math.cos(angle) * 20,
                screenY + Math.sin(angle) * 20
            );
            this.ctx.stroke();
        }
        
        // Draw player health bar
        this.drawHealthBar(screenX, screenY - 25, 30, 5, player.health, player.maxHealth, '#FF0000', '#00FF00');
        
        // Draw player hunger bar
        this.drawHealthBar(screenX, screenY - 20, 30, 3, player.hunger, player.maxHunger, '#FF8C00', '#32CD32');
        
        // Draw player thirst bar
        this.drawHealthBar(screenX, screenY - 17, 30, 3, player.thirst, player.maxThirst, '#1E90FF', '#32CD32');
        
        // Restore context state
        this.ctx.restore();
        
        // Update camera to follow player
        this.camera.x = player.position.x - this.camera.width / 2;
        this.camera.y = player.position.y - this.camera.height / 2;
    }
    
    private drawHealthBar(x: number, y: number, width: number, height: number, 
                         current: number, max: number, emptyColor: string, fullColor: string): void {
        // Draw background
        this.ctx.fillStyle = emptyColor;
        this.ctx.fillRect(x - width / 2, y, width, height);
        
        // Draw foreground
        const fillWidth = (current / max) * width;
        this.ctx.fillStyle = fullColor;
        this.ctx.fillRect(x - width / 2, y, fillWidth, height);
        
        // Draw border
        this.ctx.strokeStyle = '#000000';
        this.ctx.lineWidth = 1;
        this.ctx.strokeRect(x - width / 2, y, width, height);
    }
    
    public getCamera(): Camera {
        return this.camera;
    }
}

class Camera {
    public x: number;
    public y: number;
    public width: number;
    public height: number;
    
    constructor() {
        this.x = 0;
        this.y = 0;
        this.width = window.innerWidth;
        this.height = window.innerHeight;
    }
}