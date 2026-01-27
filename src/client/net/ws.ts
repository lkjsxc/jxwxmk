import { WorldState } from "../state/world";
import { clearToken, saveToken } from "../state/player";
import { QuestState } from "../state/quests";

export type Toast = { text: string; expiresAt: number };
export type NpcModal = { npcId: string; name: string; text: string; options: string[] } | null;

export class NetClient {
  private ws: WebSocket | null = null;
  private world: WorldState;
  private quests: QuestState;
  onToast: ((toast: Toast) => void) | null = null;
  onNpcModal: ((modal: NpcModal) => void) | null = null;
  onSessionRevoked: (() => void) | null = null;
  onWelcome: ((id: string, token: string, spawned: boolean) => void) | null = null;

  constructor(world: WorldState, quests: QuestState) {
    this.world = world;
    this.quests = quests;
  }

  connect(token: string | null): void {
    const url = token ? `/ws?token=${token}` : "/ws";
    this.ws = new WebSocket(url);
    this.ws.onmessage = (event) => this.handleMessage(event.data);
    this.ws.onclose = () => {
      this.world.chunks.clear();
    };
  }

  send(message: unknown): void {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      return;
    }
    this.ws.send(JSON.stringify(message));
  }

  sendInput(state: { dx: number; dy: number; attack: boolean; interact: boolean }): void {
    if (!state.attack && !state.interact && state.dx === 0 && state.dy === 0) {
      return;
    }
    this.send({ type: "input", data: state });
  }

  handleMessage(raw: string): void {
    let msg: any;
    try {
      msg = JSON.parse(raw);
    } catch {
      return;
    }

    switch (msg.type) {
      case "welcome":
        saveToken(msg.token);
        this.world.setLocalPlayer(msg.id);
        this.onWelcome?.(msg.id, msg.token, msg.spawned);
        if (!msg.spawned) {
          this.send({ type: "spawn", data: { settlement_id: null } });
        }
        break;
      case "sessionRevoked":
        clearToken();
        if (this.onSessionRevoked) {
          this.onSessionRevoked();
        }
        break;
      case "chunkAdd":
        this.world.applyChunkAdd(msg.data);
        break;
      case "chunkRemove":
        this.world.applyChunkRemove(msg.data);
        break;
      case "entityDelta":
        this.world.applyEntityDelta(msg.data);
        break;
      case "achievement":
        this.onToast?.({ text: `Achievement unlocked: ${msg.data.name}`, expiresAt: Date.now() + 3000 });
        break;
      case "notification":
        this.onToast?.({ text: msg.data.text, expiresAt: Date.now() + 3000 });
        break;
      case "npcInteraction":
        this.onNpcModal?.({
          npcId: msg.data.npc_id,
          name: msg.data.name,
          text: msg.data.text,
          options: msg.data.options,
        });
        break;
      case "questUpdate":
        this.quests.applyUpdate(msg.data.quest);
        break;
      default:
        break;
    }
  }
}
