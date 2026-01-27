import { ChunkAddData, EntityDeltaData, QuestState } from "../state/types";

export interface InputState {
  dx: number;
  dy: number;
  attack: boolean;
  interact: boolean;
}

export type ClientMessage =
  | { type: "input"; data: InputState }
  | { type: "spawn"; data: { settlement_id: string | null } }
  | { type: "craft"; data: { recipe: string } }
  | { type: "trade"; data: { npc_id: string; item: string; count: number; buy: boolean } }
  | { type: "npcAction"; data: { npc_id: string; option: number } }
  | { type: "acceptQuest"; data: { quest_id: string } }
  | { type: "slot"; data: { slot: number } }
  | { type: "swapSlots"; data: { from: number; to: number } }
  | { type: "name"; data: { name: string } };

export type ServerMessage =
  | {
      type: "welcome";
      id: string;
      token: string;
      version: number;
      spawned: boolean;
    }
  | { type: "sessionRevoked"; reason: string }
  | { type: "chunkAdd"; data: ChunkAddData }
  | { type: "chunkRemove"; data: { coord: [number, number] } }
  | { type: "entityDelta"; data: EntityDeltaData }
  | { type: "achievement"; data: { id: string; name: string } }
  | { type: "notification"; data: { text: string } }
  | {
      type: "npcInteraction";
      data: { npc_id: string; name: string; text: string; options: string[] };
    }
  | { type: "questUpdate"; data: { quest: QuestState } };
