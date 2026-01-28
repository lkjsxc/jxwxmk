// Message types from server
export interface WelcomeMessage {
  type: 'welcome';
  id: string;
  token: string;
  version: number;
  spawned: boolean;
}

export interface PlayerVitals {
  hp: number;
  max_hp: number;
  hunger: number;
  max_hunger: number;
  temperature: number;
  max_temperature: number;
}

export interface InventorySlot {
  item: string;
  count: number;
}

export interface PlayerStats {
  steps: number;
  kills: number;
  crafts: number;
  gathers: number;
  deaths: number;
}

export interface QuestInfo {
  id: string;
  name: string;
  state: string;
  objectives: unknown[];
}

export interface PlayerState {
  id: string;
  name: string;
  spawned: boolean;
  vitals: PlayerVitals;
  inventory: Array<InventorySlot | null>;
  active_slot: number;
  level: number;
  xp: number;
  stats: PlayerStats;
  quests: QuestInfo[];
  achievements: string[];
}

export interface PlayerUpdateMessage {
  type: 'playerUpdate';
  data: PlayerState;
}

export interface NotificationMessage {
  type: 'notification';
  data: { text: string };
}

export interface ErrorMessage {
  type: 'error';
  data: { code: string; message: string; details: unknown | null };
}

export interface CombatResultMessage {
  type: 'combatResult';
  data: {
    target_id: string;
    damage: number;
    hit: boolean;
    critical: boolean;
  };
}

export interface ResourceDepletedMessage {
  type: 'resourceDepleted';
  data: {
    resource_id: string;
    items_received: Array<{ item: string; count: number }>;
  };
}

export interface SessionRevokedMessage {
  type: 'sessionRevoked';
  reason: string;
}

export interface EntitySnapshot {
  id: string;
  kind: 'player' | 'resource' | 'mob' | 'structure' | 'npc';
  subtype?: string;
  x: number;
  y: number;
  hp?: number;
  max_hp?: number;
  level?: number;
  name?: string | null;
  range?: number | null;
}

export interface ChunkAddMessage {
  type: 'chunkAdd';
  data: {
    coord: [number, number];
    biome: string;
    entities: {
      resources: Record<string, EntitySnapshot>;
      mobs: Record<string, EntitySnapshot>;
      structures: Record<string, EntitySnapshot>;
      npcs: Record<string, EntitySnapshot>;
    };
  };
}

export interface ChunkRemoveMessage {
  type: 'chunkRemove';
  data: { coord: [number, number] };
}

export interface EntityDeltaMessage {
  type: 'entityDelta';
  data: {
    chunk: [number, number];
    updates: EntitySnapshot[];
    removes: Array<{ id: string; kind: string }>;
  };
}

export interface AchievementMessage {
  type: 'achievement';
  data: { id: string; name: string };
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
  data: { quest: QuestInfo };
}

export type ServerMessage =
  | WelcomeMessage
  | PlayerUpdateMessage
  | NotificationMessage
  | ErrorMessage
  | CombatResultMessage
  | ResourceDepletedMessage
  | SessionRevokedMessage
  | ChunkAddMessage
  | ChunkRemoveMessage
  | EntityDeltaMessage
  | AchievementMessage
  | NpcInteractionMessage
  | QuestUpdateMessage;

// Client message types
export interface InputMessage {
  type: 'input';
  data: {
    dx: number;
    dy: number;
    attack: boolean;
    interact: boolean;
    aim: { x: number; y: number } | null;
  };
}

// World state
export interface Chunk {
  coord: [number, number];
  biome: string;
  entities: Map<string, EntitySnapshot>;
}

// Input state
export interface JoystickState {
  active: boolean;
  touchId: number | null;
  centerX: number;
  centerY: number;
  dx: number;
  dy: number;
}

export interface PointerState {
  x: number;
  y: number;
  down: boolean;
  startTime: number;
  longPressTriggered: boolean;
  touchId: number | null;
}
