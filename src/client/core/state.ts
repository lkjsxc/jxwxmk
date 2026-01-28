import type { PlayerState, Chunk, EntitySnapshot } from '../types';

export type StateChangeHandler = () => void;

export class GameState {
  private playerState: PlayerState | null = null;
  private chunks = new Map<string, Chunk>();
  private localPlayerPos = { x: 0, y: 0 };
  private handlers: Set<StateChangeHandler> = new Set();

  // Player state
  setPlayerState(state: PlayerState): void {
    this.playerState = state;
    this.notify();
  }

  getPlayerState(): PlayerState | null {
    return this.playerState;
  }

  isSpawned(): boolean {
    return this.playerState?.spawned ?? false;
  }

  // Local position tracking for rendering
  setLocalPosition(x: number, y: number): void {
    this.localPlayerPos.x = x;
    this.localPlayerPos.y = y;
  }

  getLocalPosition(): { x: number; y: number } {
    return { ...this.localPlayerPos };
  }

  // Chunk management
  addChunk(chunk: Chunk): void {
    const key = chunkKey(chunk.coord);
    this.chunks.set(key, chunk);
  }

  removeChunk(coord: [number, number]): void {
    this.chunks.delete(chunkKey(coord));
  }

  getChunk(coord: [number, number]): Chunk | undefined {
    return this.chunks.get(chunkKey(coord));
  }

  getAllChunks(): Chunk[] {
    return Array.from(this.chunks.values());
  }

  updateEntities(
    chunkCoord: [number, number],
    updates: EntitySnapshot[],
    removes: Array<{ id: string; kind: string }>
  ): void {
    const chunk = this.getChunk(chunkCoord);
    if (!chunk) return;

    // Apply updates
    for (const entity of updates) {
      chunk.entities.set(entity.id, entity);
    }

    // Apply removes
    for (const remove of removes) {
      chunk.entities.delete(remove.id);
    }
  }

  getAllEntities(): EntitySnapshot[] {
    const entities: EntitySnapshot[] = [];
    for (const chunk of this.chunks.values()) {
      for (const entity of chunk.entities.values()) {
        entities.push(entity);
      }
    }
    return entities;
  }

  // Subscription
  subscribe(handler: StateChangeHandler): () => void {
    this.handlers.add(handler);
    return () => this.handlers.delete(handler);
  }

  private notify(): void {
    this.handlers.forEach((h) => h());
  }
}

function chunkKey(coord: [number, number]): string {
  return `${coord[0]},${coord[1]}`;
}
