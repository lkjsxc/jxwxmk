import { camera } from './camera';
import { world } from './world';
import { input } from './input';
import type { Entity, PlayerState } from './types';

const PPU = 16; // pixels per world unit at zoom 1.0

// Entity colors by kind/subtype
const ENTITY_COLORS: Record<string, string> = {
  player: '#6a6aff',
  tree: '#2d5a27',
  rock: '#666666',
  berry_bush: '#8b4513',
  wolf: '#8b0000',
  bear: '#5c4033',
  wall: '#8b7355',
  door: '#654321',
  workbench: '#a0522d',
  torch: '#ff8c00',
  npc: '#ffd700',
};

// Biome background colors
const BIOME_COLORS: Record<string, string> = {
  forest: '#1a2f1a',
  plains: '#2f3f1a',
  desert: '#3f3f1a',
  tundra: '#2a3a4a',
};

export class Renderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private animationId: number | null = null;
  private lastHitEntities = new Map<string, number>();

  constructor() {
    this.canvas = document.getElementById('game-canvas') as HTMLCanvasElement;
    this.ctx = this.canvas.getContext('2d')!;

    this.resize();
    window.addEventListener('resize', () => this.resize());
  }

  private resize(): void {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }

  start(): void {
    if (this.animationId !== null) return;
    this.loop();
  }

  stop(): void {
    if (this.animationId !== null) {
      cancelAnimationFrame(this.animationId);
      this.animationId = null;
    }
  }

  private loop = (): void => {
    this.animationId = requestAnimationFrame(this.loop);
    this.render();
  };

  private render(): void {
    const ctx = this.ctx;
    const width = this.canvas.width;
    const height = this.canvas.height;

    // Clear canvas
    ctx.fillStyle = '#0f0f1a';
    ctx.fillRect(0, 0, width, height);

    // Update camera
    camera.update();
    input.setCamera(camera.x, camera.y, camera.zoom);

    // Draw background grid
    this.drawGrid(width, height);

    // Draw entities
    const entities = world.getVisibleEntities();
    const now = Date.now();

    // Sort by Y for depth
    entities.sort((a, b) => a.y - b.y);

    for (const entity of entities) {
      this.drawEntity(entity, now);
    }

    // Draw targeting highlight
    this.drawTargetingHighlight(width, height);

    // Draw joystick if active (touch)
    this.drawJoystick();
  }

  private drawGrid(width: number, viewportHeight: number): void {
    const ctx = this.ctx;
    const viewport = camera.getViewport(width, viewportHeight);

    ctx.strokeStyle = '#1a1a2e';
    ctx.lineWidth = 1;

    // Grid size in world units
    const gridSize = 10;

    const startX = Math.floor(viewport.minX / gridSize) * gridSize;
    const endX = Math.ceil(viewport.maxX / gridSize) * gridSize;
    const startY = Math.floor(viewport.minY / gridSize) * gridSize;
    const endY = Math.ceil(viewport.maxY / gridSize) * gridSize;

    ctx.beginPath();
    for (let x = startX; x <= endX; x += gridSize) {
      const screenX = camera.worldToScreen(x, 0, width, viewportHeight).x;
      ctx.moveTo(screenX, 0);
      ctx.lineTo(screenX, viewportHeight);
    }
    for (let y = startY; y <= endY; y += gridSize) {
      const screenY = camera.worldToScreen(0, y, width, viewportHeight).y;
      ctx.moveTo(0, screenY);
      ctx.lineTo(width, screenY);
    }
    ctx.stroke();
  }

  private drawEntity(entity: Entity, now: number): void {
    const ctx = this.ctx;
    const width = this.canvas.width;
    const height = this.canvas.height;

    // Get interpolated position
    const pos = world.getInterpolatedPosition(entity, now);
    const screenPos = camera.worldToScreen(pos.x, pos.y, width, height);

    // Calculate size based on entity type
    let radiusWu = 0.75; // Default player radius
    if (entity.kind === 'resource') radiusWu = 1.0;
    if (entity.kind === 'mob') radiusWu = 0.9;
    if (entity.kind === 'npc') radiusWu = 0.8;
    if (entity.kind === 'structure') radiusWu = 1.25;

    const radiusPx = radiusWu * PPU * camera.zoom;

    // Get color
    let color = ENTITY_COLORS[entity.subtype] || ENTITY_COLORS[entity.kind] || '#888';

    // Hit flash animation
    const lastHit = this.lastHitEntities.get(entity.id);
    if (lastHit && now - lastHit < 250) {
      const t = (now - lastHit) / 250;
      const scale = 1 + Math.sin(t * Math.PI) * 0.15;
      ctx.save();
      ctx.translate(screenPos.x, screenPos.y);
      ctx.scale(scale, scale);
      ctx.translate(-screenPos.x, -screenPos.y);
      this.lastHitEntities.set(entity.id, lastHit); // Keep tracking until animation ends
    }

    // Draw entity body
    ctx.fillStyle = color;
    ctx.beginPath();
    ctx.arc(screenPos.x, screenPos.y, radiusPx, 0, Math.PI * 2);
    ctx.fill();

    // Draw outline
    ctx.strokeStyle = '#000';
    ctx.lineWidth = 2;
    ctx.stroke();

    // Draw health bar if damaged
    if (entity.hp !== undefined && entity.max_hp !== undefined && entity.hp < entity.max_hp) {
      const barWidth = radiusPx * 2;
      const barHeight = 4;
      const hpPct = entity.hp / entity.max_hp;

      ctx.fillStyle = '#333';
      ctx.fillRect(screenPos.x - barWidth / 2, screenPos.y - radiusPx - 10, barWidth, barHeight);

      ctx.fillStyle = hpPct > 0.5 ? '#0f0' : hpPct > 0.25 ? '#ff0' : '#f00';
      ctx.fillRect(screenPos.x - barWidth / 2, screenPos.y - radiusPx - 10, barWidth * hpPct, barHeight);
    }

    // Draw level indicator for mobs
    if (entity.kind === 'mob' && entity.level !== undefined) {
      ctx.fillStyle = '#fff';
      ctx.font = `bold ${10 * camera.zoom}px sans-serif`;
      ctx.textAlign = 'center';
      ctx.fillText(`Lv.${entity.level}`, screenPos.x, screenPos.y - radiusPx - 15);
    }

    // Draw name for NPCs and players
    if ((entity.kind === 'npc' || entity.kind === 'player') && entity.name) {
      ctx.fillStyle = '#fff';
      ctx.font = `${12 * camera.zoom}px sans-serif`;
      ctx.textAlign = 'center';
      ctx.fillText(entity.name, screenPos.x, screenPos.y - radiusPx - 15);
    }

    if (lastHit && now - lastHit < 250) {
      ctx.restore();
    }
  }

  private drawTargetingHighlight(width: number, height: number): void {
    const ctx = this.ctx;

    // Get aim position
    const aim = camera.screenToWorld(input.mouseX, input.mouseY, width, height);

    // Find closest entity within range
    const nearby = world.getEntitiesNear(aim.x, aim.y, 4.0);
    const target = nearby[0];

    if (target) {
      const pos = world.getInterpolatedPosition(target, Date.now());
      const screenPos = camera.worldToScreen(pos.x, pos.y, width, height);
      let radiusWu = 0.75;
      if (target.kind === 'resource') radiusWu = 1.0;
      if (target.kind === 'mob') radiusWu = 0.9;
      if (target.kind === 'npc') radiusWu = 0.8;
      if (target.kind === 'structure') radiusWu = 1.25;
      const radiusPx = radiusWu * PPU * camera.zoom;

      // Draw highlight ring
      ctx.strokeStyle = '#6a6aff';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.arc(screenPos.x, screenPos.y, radiusPx + 4, 0, Math.PI * 2);
      ctx.stroke();

      // Draw tooltip
      const tooltipText = this.getInteractionText(target);
      ctx.fillStyle = 'rgba(0, 0, 0, 0.8)';
      ctx.font = '12px sans-serif';
      const textWidth = ctx.measureText(tooltipText).width;
      ctx.fillRect(screenPos.x - textWidth / 2 - 4, screenPos.y - radiusPx - 35, textWidth + 8, 18);
      ctx.fillStyle = '#fff';
      ctx.textAlign = 'center';
      ctx.fillText(tooltipText, screenPos.x, screenPos.y - radiusPx - 22);
    }
  }

  private getInteractionText(entity: Entity): string {
    if (entity.kind === 'resource') return `Gather ${entity.subtype}`;
    if (entity.kind === 'mob') return 'Attack';
    if (entity.kind === 'npc') return 'Talk';
    if (entity.kind === 'structure') return 'Interact';
    return 'Interact';
  }

  private drawJoystick(): void {
    if (!input['joystickActive']) return;

    const ctx = this.ctx;
    const centerX = input['joystickCenterX'];
    const centerY = input['joystickCenterY'];
    const currentX = input['joystickCurrentX'];
    const currentY = input['joystickCurrentY'];

    // Outer ring
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.3)';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.arc(centerX, centerY, 50, 0, Math.PI * 2);
    ctx.stroke();

    // Inner thumb
    ctx.fillStyle = 'rgba(255, 255, 255, 0.5)';
    ctx.beginPath();
    ctx.arc(currentX, currentY, 20, 0, Math.PI * 2);
    ctx.fill();
  }

  triggerHitFlash(entityId: string): void {
    this.lastHitEntities.set(entityId, Date.now());
  }
}

export const renderer = new Renderer();
