import type { Chunk, Entity, EntityKind, EntityDeltaMessage } from './types';

const CHUNK_SIZE = 128; // world units per chunk

export class WorldManager {
  private chunks = new Map<string, Chunk>(); // key: "x,y"
  private playerEntity: Entity | null = null;
  private playerMissingSince: number | null = null;
  private readonly playerTimeoutMs = 3000; // 3 seconds before game over

  // Get chunk key from coordinates
  private getChunkKey(coord: [number, number]): string {
    return `${coord[0]},${coord[1]}`;
  }

  // Get chunk coordinates from world position
  getChunkCoord(worldX: number, worldY: number): [number, number] {
    const cx = Math.floor(worldX / CHUNK_SIZE);
    const cy = Math.floor(worldY / CHUNK_SIZE);
    return [cx, cy];
  }

  // Add or update a chunk
  addChunk(chunk: Chunk): void {
    const key = this.getChunkKey(chunk.coord);
    this.chunks.set(key, chunk);
  }

  // Remove a chunk
  removeChunk(coord: [number, number]): void {
    const key = this.getChunkKey(coord);
    this.chunks.delete(key);
  }

  // Apply entity delta to a chunk
  applyEntityDelta(delta: EntityDeltaMessage['data']): void {
    const key = this.getChunkKey(delta.chunk);
    const chunk = this.chunks.get(key);
    if (!chunk) return;

    // Apply updates
    for (const update of delta.updates) {
      // Store previous position for interpolation
      const existing = this.findEntity(update.id);
      if (existing) {
        update.prevX = existing.x;
        update.prevY = existing.y;
        update.lastUpdate = Date.now();
      }

      // Add to appropriate collection
      this.addEntityToChunk(chunk, update);
    }

    // Apply removals
    for (const removal of delta.removes) {
      this.removeEntityFromChunk(chunk, removal.id, removal.kind);
    }
  }

  private findEntity(id: string): Entity | null {
    for (const chunk of this.chunks.values()) {
      for (const collection of [chunk.entities.resources, chunk.entities.mobs, chunk.entities.structures, chunk.entities.npcs]) {
        const found = collection.find(e => e.id === id);
        if (found) return found;
      }
    }
    return null;
  }

  private addEntityToChunk(chunk: Chunk, entity: Entity): void {
    const collection = this.getEntityCollection(chunk, entity.kind);
    if (collection) {
      // Remove existing entity with same ID if present
      const idx = collection.findIndex(e => e.id === entity.id);
      if (idx >= 0) {
        collection[idx] = entity;
      } else {
        collection.push(entity);
      }
    }

    // Track player entity separately
    if (entity.kind === 'player' && entity.id === this.playerEntity?.id) {
      this.playerEntity = entity;
      this.playerMissingSince = null;
    }
  }

  private removeEntityFromChunk(chunk: Chunk, id: string, kind: EntityKind): void {
    const collection = this.getEntityCollection(chunk, kind);
    if (collection) {
      const idx = collection.findIndex(e => e.id === id);
      if (idx >= 0) {
        collection.splice(idx, 1);
      }
    }

    // Check if player was removed
    if (kind === 'player' && id === this.playerEntity?.id) {
      this.playerMissingSince = Date.now();
    }
  }

  private getEntityCollection(chunk: Chunk, kind: EntityKind): Entity[] | null {
    switch (kind) {
      case 'resource': return chunk.entities.resources;
      case 'mob': return chunk.entities.mobs;
      case 'structure': return chunk.entities.structures;
      case 'npc': return chunk.entities.npcs;
      case 'player': return []; // Players handled separately
      default: return null;
    }
  }

  // Get all visible entities (for rendering)
  getVisibleEntities(): Entity[] {
    const entities: Entity[] = [];
    for (const chunk of this.chunks.values()) {
      for (const collection of [chunk.entities.resources, chunk.entities.mobs, chunk.entities.structures, chunk.entities.npcs]) {
        entities.push(...collection);
      }
    }
    if (this.playerEntity) {
      entities.push(this.playerEntity);
    }
    return entities;
  }

  // Get entities near a point (for targeting)
  getEntitiesNear(x: number, y: number, radius: number): Entity[] {
    const results: Entity[] = [];
    const entities = this.getVisibleEntities();

    for (const entity of entities) {
      const dx = entity.x - x;
      const dy = entity.y - y;
      const dist = Math.sqrt(dx * dx + dy * dy);
      if (dist <= radius) {
        results.push(entity);
      }
    }

    return results.sort((a, b) => {
      const da = Math.sqrt((a.x - x) ** 2 + (a.y - y) ** 2);
      const db = Math.sqrt((b.x - x) ** 2 + (b.y - y) ** 2);
      return da - db;
    });
  }

  // Set player entity ID for tracking
  setPlayerId(id: string): void {
    this.playerEntity = { id, kind: 'player', subtype: 'player', x: 0, y: 0 };
  }

  // Get player entity
  getPlayerEntity(): Entity | null {
    return this.playerEntity;
  }

  // Check if player should be in game over state
  isGameOver(): boolean {
    return this.playerMissingSince !== null &&
           Date.now() - this.playerMissingSince > this.playerTimeoutMs;
  }

  // Reset game over state
  resetGameOver(): void {
    this.playerMissingSince = null;
  }

  // Clear all chunks (on disconnect)
  clear(): void {
    this.chunks.clear();
    this.playerEntity = null;
    this.playerMissingSince = null;
  }

  // Get interpolated entity position
  getInterpolatedPosition(entity: Entity, now: number): { x: number; y: number } {
    if (entity.prevX === undefined || entity.lastUpdate === undefined) {
      return { x: entity.x, y: entity.y };
    }

    const elapsed = now - entity.lastUpdate;
    const duration = 100; // Interpolate over 100ms
    const t = Math.min(1, elapsed / duration);

    return {
      x: entity.prevX + (entity.x - entity.prevX) * t,
      y: entity.prevY + (entity.y - entity.prevY) * t,
    };
  }
}

export const world = new WorldManager();
