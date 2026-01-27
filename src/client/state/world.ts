export interface EntitySnapshot {
  id: string;
  kind: string;
  subtype: string;
  x: number;
  y: number;
  hp?: number;
  max_hp?: number;
  level?: number;
  name?: string | null;
  range?: number | null;
  prevX?: number;
  prevY?: number;
  lastUpdate?: number;
}

export interface ChunkState {
  coord: [number, number];
  biome: string;
  entities: Map<string, EntitySnapshot>;
}

export class WorldState {
  chunks = new Map<string, ChunkState>();
  localPlayerId: string | null = null;
  lastPlayerSeenMs = 0;

  applyChunkAdd(data: { coord: [number, number]; biome: string; entities: any }): void {
    const key = `${data.coord[0]},${data.coord[1]}`;
    const entities = new Map<string, EntitySnapshot>();
    const merge = (group: Record<string, EntitySnapshot>) => {
      Object.values(group || {}).forEach((entity) => {
        entities.set(entity.id, { ...entity, prevX: entity.x, prevY: entity.y, lastUpdate: Date.now() });
      });
    };
    merge(data.entities?.resources || {});
    merge(data.entities?.mobs || {});
    merge(data.entities?.structures || {});
    merge(data.entities?.npcs || {});
    this.chunks.set(key, {
      coord: data.coord,
      biome: data.biome,
      entities,
    });
  }

  applyChunkRemove(data: { coord: [number, number] }): void {
    const key = `${data.coord[0]},${data.coord[1]}`;
    this.chunks.delete(key);
  }

  applyEntityDelta(data: { chunk: [number, number]; updates: EntitySnapshot[]; removes: any[] }): void {
    const key = `${data.chunk[0]},${data.chunk[1]}`;
    const chunk = this.chunks.get(key);
    if (!chunk) {
      return;
    }
    data.removes.forEach((removal) => {
      chunk.entities.delete(removal.id);
    });
    data.updates.forEach((update) => {
      const existing = chunk.entities.get(update.id);
      const prevX = existing?.x ?? update.x;
      const prevY = existing?.y ?? update.y;
      chunk.entities.set(update.id, {
        ...update,
        prevX,
        prevY,
        lastUpdate: Date.now(),
      });
      if (update.kind === "player" && update.id === this.localPlayerId) {
        this.lastPlayerSeenMs = Date.now();
      }
    });
  }

  setLocalPlayer(id: string): void {
    this.localPlayerId = id;
    this.lastPlayerSeenMs = Date.now();
  }

  getLocalPlayer(): EntitySnapshot | null {
    if (!this.localPlayerId) {
      return null;
    }
    for (const chunk of this.chunks.values()) {
      const entity = chunk.entities.get(this.localPlayerId);
      if (entity) {
        return entity;
      }
    }
    return null;
  }

  isGameOver(): boolean {
    if (!this.localPlayerId) {
      return false;
    }
    return Date.now() - this.lastPlayerSeenMs > 4000;
  }
}
