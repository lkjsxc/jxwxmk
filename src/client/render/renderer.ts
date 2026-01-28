import type { Camera } from './camera';
import type { GameState } from '../core/state';
import type { EntitySnapshot, PlayerState } from '../types';

export class Renderer {
  private ctx: CanvasRenderingContext2D;
  private camera: Camera;
  private state: GameState;

  constructor(
    private canvas: HTMLCanvasElement,
    camera: Camera,
    state: GameState
  ) {
    const ctx = canvas.getContext('2d');
    if (!ctx) throw new Error('Failed to get canvas context');
    this.ctx = ctx;
    this.camera = camera;
    this.state = state;

    this.setupResize();
  }

  private setupResize(): void {
    const resize = () => {
      this.canvas.width = window.innerWidth;
      this.canvas.height = window.innerHeight;
    };
    resize();
    window.addEventListener('resize', resize);
  }

  render(): void {
    const ctx = this.ctx;
    const canvas = this.canvas;

    // Clear background
    ctx.fillStyle = '#0f0f1e';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw grid
    this.drawGrid();

    // Draw chunks/entities
    this.drawWorld();

    // Draw joystick (if active)
    this.drawJoystick();
  }

  private drawGrid(): void {
    const ctx = this.ctx;
    const scale = this.camera.getScale();
    const gridSize = 16 * scale; // 1 world unit = 16 pixels at zoom 1.0

    // Calculate visible grid bounds
    const offsetX = (-this.camera.x * scale + this.canvas.width / 2) % gridSize;
    const offsetY = (-this.camera.y * scale + this.canvas.height / 2) % gridSize;

    ctx.strokeStyle = 'rgba(100, 100, 150, 0.15)';
    ctx.lineWidth = 1;

    ctx.beginPath();
    for (let x = offsetX; x < this.canvas.width; x += gridSize) {
      ctx.moveTo(x, 0);
      ctx.lineTo(x, this.canvas.height);
    }
    for (let y = offsetY; y < this.canvas.height; y += gridSize) {
      ctx.moveTo(0, y);
      ctx.lineTo(this.canvas.width, y);
    }
    ctx.stroke();
  }

  private drawWorld(): void {
    const player = this.state.getPlayerState();
    if (!player?.spawned) return;

    // Get all entities from chunks
    const entities = this.state.getAllEntities();

    // Sort by Y for simple depth
    entities.sort((a, b) => a.y - b.y);

    // Draw entities
    for (const entity of entities) {
      this.drawEntity(entity);
    }

    // Draw local player
    this.drawLocalPlayer();
  }

  private drawEntity(entity: EntitySnapshot): void {
    const ctx = this.ctx;
    const pos = this.camera.worldToScreen(
      entity.x,
      entity.y,
      this.canvas.width,
      this.canvas.height
    );
    const scale = this.camera.getScale();

    // Skip if off-screen
    if (
      pos.x < -50 ||
      pos.x > this.canvas.width + 50 ||
      pos.y < -50 ||
      pos.y > this.canvas.height + 50
    ) {
      return;
    }

    const radius = 8 * (this.camera.zoom / 1.1);

    // Draw based on kind
    switch (entity.kind) {
      case 'player':
        this.drawCircle(pos.x, pos.y, radius, '#6a6aff');
        if (entity.name) {
          this.drawLabel(pos.x, pos.y - radius - 5, entity.name);
        }
        break;
      case 'resource':
        this.drawResource(entity.subtype || '', pos.x, pos.y, radius, scale);
        break;
      case 'mob':
        this.drawCircle(pos.x, pos.y, radius, '#ff6a6a');
        if (entity.name) {
          this.drawLabel(pos.x, pos.y - radius - 5, entity.name);
        }
        break;
      case 'structure':
        this.drawRect(pos.x - radius, pos.y - radius, radius * 2, radius * 2, '#8a8aaa');
        break;
      case 'npc':
        this.drawCircle(pos.x, pos.y, radius, '#ffaa44');
        if (entity.name) {
          this.drawLabel(pos.x, pos.y - radius - 5, entity.name);
        }
        break;
    }

    // Draw HP bar if damaged
    if (entity.hp !== undefined && entity.max_hp !== undefined && entity.hp < entity.max_hp) {
      this.drawHPBar(pos.x, pos.y - radius - 12, radius * 2, 4, entity.hp, entity.max_hp);
    }
  }

  private drawResource(subtype: string, x: number, y: number, radius: number, scale: number): void {
    // Different resources have different colors/shapes
    const colors: Record<string, string> = {
      tree: '#4a8a4a',
      stone: '#8a8a8a',
      bush: '#6aaa4a',
      ore: '#6a6a8a',
    };

    const color = colors[subtype] || '#aaaaaa';

    // Trees are taller
    if (subtype === 'tree') {
      this.drawRect(x - radius * 0.5, y - radius * 1.5, radius, radius * 2.5, color);
    } else {
      this.drawCircle(x, y, radius, color);
    }
  }

  private drawLocalPlayer(): void {
    const pos = this.camera.worldToScreen(
      this.camera.x,
      this.camera.y,
      this.canvas.width,
      this.canvas.height
    );
    const radius = 10 * (this.camera.zoom / 1.1);

    // Draw player as larger blue circle
    this.drawCircle(pos.x, pos.y, radius, '#6a6aff');

    // Draw highlight ring
    this.ctx.strokeStyle = '#ffffff';
    this.ctx.lineWidth = 2;
    this.ctx.beginPath();
    this.ctx.arc(pos.x, pos.y, radius + 2, 0, Math.PI * 2);
    this.ctx.stroke();
  }

  private drawCircle(x: number, y: number, radius: number, color: string): void {
    this.ctx.fillStyle = color;
    this.ctx.beginPath();
    this.ctx.arc(x, y, radius, 0, Math.PI * 2);
    this.ctx.fill();
  }

  private drawRect(x: number, y: number, width: number, height: number, color: string): void {
    this.ctx.fillStyle = color;
    this.ctx.fillRect(x, y, width, height);
  }

  private drawLabel(x: number, y: number, text: string): void {
    this.ctx.fillStyle = '#ffffff';
    this.ctx.font = '10px sans-serif';
    this.ctx.textAlign = 'center';
    this.ctx.fillText(text, x, y);
  }

  private drawHPBar(
    x: number,
    y: number,
    width: number,
    height: number,
    hp: number,
    maxHP: number
  ): void {
    const pct = hp / maxHP;

    // Background
    this.ctx.fillStyle = '#333';
    this.ctx.fillRect(x - width / 2, y, width, height);

    // Fill
    this.ctx.fillStyle = pct > 0.5 ? '#4a4' : pct > 0.25 ? '#aa4' : '#a44';
    this.ctx.fillRect(x - width / 2, y, width * pct, height);
  }

  private drawJoystick(): void {
    // Joystick is drawn by the input manager
    // This is just a placeholder if we want canvas-based joystick
  }
}
