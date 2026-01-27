export type EntityKind = "player" | "resource" | "mob" | "structure" | "npc";

export interface EntityUpdate {
  id: string;
  kind: EntityKind;
  subtype: string;
  x: number;
  y: number;
  hp?: number;
  max_hp?: number;
  level?: number;
  name?: string;
  range?: number;
}

export interface ChunkEntities {
  resources: Record<string, EntityUpdate>;
  mobs: Record<string, EntityUpdate>;
  structures: Record<string, EntityUpdate>;
  npcs: Record<string, EntityUpdate>;
}

export interface ChunkAddData {
  coord: [number, number];
  biome: string;
  entities: ChunkEntities;
}

export interface EntityDeltaData {
  chunk: [number, number];
  updates: EntityUpdate[];
  removes: { id: string; kind: EntityKind }[];
}

export interface QuestObjective {
  kind: string;
  target: string;
  count: number;
  current: number;
}

export interface QuestState {
  id: string;
  name: string;
  state: string;
  objectives: QuestObjective[];
}
