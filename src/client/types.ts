// Protocol message types - matches docs/technical/backend/server/protocol.md

// ========== Entity Types ==========

export type EntityKind = 'player' | 'resource' | 'mob' | 'structure' | 'npc';

export interface Entity {
  id: string;
  kind: EntityKind;
  subtype: string;
  x: number;
  y: number;
  hp?: number;
  max_hp?: number;
  level?: number;
  name?: string | null;
  range?: number | null;
  // Client-side interpolation
  prevX?: number;
  prevY?: number;
  lastUpdate?: number;
}

// ========== Chunk Types ==========

// Server sends entities as arrays, we convert to records on the client
export interface ChunkEntities {
  resources: Entity[];
  mobs: Entity[];
  structures: Entity[];
  npcs: Entity[];
}

export interface Chunk {
  coord: [number, number];
  biome: string;
  entities: ChunkEntities;
}

// ========== Player State ==========

export interface ItemSlot {
  item: string;
  count: number;
}

export interface Vitals {
  hp: number;
  max_hp: number;
  hunger: number;
  max_hunger: number;
  temperature: number;
  max_temperature: number;
}

export interface Stats {
  steps: number;
  kills: number;
  crafts: number;
  gathers: number;
  deaths: number;
}

export interface QuestObjective {
  description: string;
  current: number;
  target: number;
}

export interface Quest {
  id: string;
  name: string;
  state: 'NotStarted' | 'InProgress' | 'ReadyToTurnIn' | 'Completed';
  description?: string;
  objectives?: QuestObjective[];
}

export interface PlayerState {
  id: string;
  name: string;
  spawned: boolean;
  x: number;
  y: number;
  vitals: Vitals;
  inventory: (ItemSlot | null)[];
  active_slot: number;
  level: number;
  xp: number;
  stats: Stats;
  quests: Quest[];
  achievements: string[];
}

// ========== Client -> Server Messages ==========

export interface InputMessage {
  type: 'input';
  data: {
    dx: number;
    dy: number;
    attack: boolean;
    interact: boolean;
    aim?: { x: number; y: number };
  };
}

export interface SpawnMessage {
  type: 'spawn';
  data: { settlement_id: string | null };
}

export interface CraftMessage {
  type: 'craft';
  data: { recipe: string };
}

export interface SlotMessage {
  type: 'slot';
  data: { slot: number };
}

export interface SwapSlotsMessage {
  type: 'swapSlots';
  data: { from: number; to: number };
}

export interface NameMessage {
  type: 'name';
  data: { name: string };
}

export interface AcceptQuestMessage {
  type: 'acceptQuest';
  data: { quest_id: string };
}

export interface NpcActionMessage {
  type: 'npcAction';
  data: { npc_id: string; option: number };
}

export type ClientMessage =
  | InputMessage
  | SpawnMessage
  | CraftMessage
  | SlotMessage
  | SwapSlotsMessage
  | NameMessage
  | AcceptQuestMessage
  | NpcActionMessage;

// ========== Server -> Client Messages ==========

export interface WelcomeMessage {
  type: 'welcome';
  id: string;
  token: string;
  version: number;
  spawned: boolean;
}

export interface SessionRevokedMessage {
  type: 'sessionRevoked';
  reason: string;
}

export interface PlayerUpdateMessage {
  type: 'playerUpdate';
  data: PlayerState;
}

export interface ChunkAddMessage {
  type: 'chunkAdd';
  data: Chunk;
}

export interface ChunkRemoveMessage {
  type: 'chunkRemove';
  data: { coord: [number, number] };
}

export interface EntityDeltaMessage {
  type: 'entityDelta';
  data: {
    chunk: [number, number];
    updates: Entity[];
    removes: Array<{ id: string; kind: EntityKind }>;
  };
}

export interface AchievementMessage {
  type: 'achievement';
  data: { id: string; name: string };
}

export interface NotificationMessage {
  type: 'notification';
  data: { text: string };
}

export interface ErrorMessage {
  type: 'error';
  data: {
    code: string;
    message: string;
    details?: unknown;
  };
}

export interface NpcInteractionMessage {
  type: 'npcInteraction';
  data: {
    npc_id: string;
    name: string;
    text: string;
    options: string[];
  };
}

export interface QuestUpdateMessage {
  type: 'questUpdate';
  data: { quest: Quest };
}

export type ServerMessage =
  | WelcomeMessage
  | SessionRevokedMessage
  | PlayerUpdateMessage
  | ChunkAddMessage
  | ChunkRemoveMessage
  | EntityDeltaMessage
  | AchievementMessage
  | NotificationMessage
  | ErrorMessage
  | NpcInteractionMessage
  | QuestUpdateMessage;

// ========== UI State ==========

export type GameScreen = 'login' | 'game' | 'gameover' | 'session_revoked';

export interface Notification {
  id: number;
  text: string;
  type: 'info' | 'error' | 'achievement';
  createdAt: number;
}

// ========== Crafting Recipes ==========

export interface Recipe {
  id: string;
  name: string;
  requirements: Array<{ item: string; count: number }>;
}

export const RECIPES: Recipe[] = [
  { id: 'WoodPickaxe', name: 'Wood Pick', requirements: [{ item: 'wood', count: 10 }] },
  { id: 'StonePickaxe', name: 'Stone Pick', requirements: [{ item: 'wood', count: 10 }, { item: 'stone', count: 10 }] },
  { id: 'WoodWall', name: 'Wood Wall', requirements: [{ item: 'wood', count: 20 }] },
  { id: 'Door', name: 'Door', requirements: [{ item: 'wood', count: 30 }] },
  { id: 'Torch', name: 'Torch', requirements: [{ item: 'wood', count: 2 }] },
  { id: 'Workbench', name: 'Workbench', requirements: [{ item: 'wood', count: 50 }] },
];

// ========== Achievement List ==========

export const ALL_ACHIEVEMENTS = [
  { id: 'first_steps', name: 'First Steps' },
  { id: 'gatherer', name: 'Gatherer' },
  { id: 'craftsman', name: 'Craftsman' },
  { id: 'survivor', name: 'Survivor' },
  { id: 'warrior', name: 'Warrior' },
];
