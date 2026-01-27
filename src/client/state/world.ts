import { ChunkAddData, EntityDeltaData, EntityUpdate } from "./types";

const chunkKey = (coord: [number, number]) => `${coord[0]},${coord[1]}`;

export class WorldState {
  chunks = new Map<string, ChunkAddData>();
  players = new Map<string, EntityUpdate>();

  addChunk(data: ChunkAddData) {
    this.chunks.set(chunkKey(data.coord), data);
  }

  removeChunk(coord: [number, number]) {
    this.chunks.delete(chunkKey(coord));
  }

  applyDelta(delta: EntityDeltaData) {
    const key = chunkKey(delta.chunk);
    const chunk = this.chunks.get(key);
    if (!chunk) {
      return;
    }
    for (const update of delta.updates) {
      if (update.kind === "player") {
        this.players.set(update.id, update);
        continue;
      }
      const bucket = this.bucket(chunk, update.kind);
      bucket[update.id] = update;
    }
    for (const remove of delta.removes) {
      if (remove.kind === "player") {
        this.players.delete(remove.id);
        continue;
      }
      const bucket = this.bucket(chunk, remove.kind);
      delete bucket[remove.id];
    }
  }

  private bucket(chunk: ChunkAddData, kind: string): Record<string, EntityUpdate> {
    if (kind === "resource") return chunk.entities.resources;
    if (kind === "mob") return chunk.entities.mobs;
    if (kind === "structure") return chunk.entities.structures;
    return chunk.entities.npcs;
  }
}
