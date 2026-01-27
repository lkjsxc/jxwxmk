import { ClientMessage, ServerMessage } from "./protocol";
import { PlayerSession } from "../state/player";
import { WorldState } from "../state/world";
import { UIManager } from "../ui/ui_manager";

export class SocketClient {
  private ws: WebSocket | null = null;
  private inputInterval: number | null = null;

  connect(session: PlayerSession, world: WorldState, ui: UIManager) {
    const token = localStorage.getItem("session_token");
    const url = token ? `/ws?token=${token}` : "/ws";
    this.ws = new WebSocket(url);

    this.ws.onmessage = (event) => {
      const message = JSON.parse(event.data) as ServerMessage;
      switch (message.type) {
        case "welcome":
          session.setWelcome(message.id, message.token, message.spawned);
          localStorage.setItem("session_token", message.token);
          if (!message.spawned) {
            this.send({ type: "spawn", data: { settlement_id: null } });
          }
          break;
        case "sessionRevoked":
          session.sessionRevoked = true;
          session.token = null;
          localStorage.removeItem("session_token");
          ui.setSessionRevoked(true);
          break;
        case "chunkAdd":
          world.addChunk(message.data);
          break;
        case "chunkRemove":
          world.removeChunk(message.data.coord);
          break;
        case "entityDelta":
          world.applyDelta(message.data);
          break;
        case "achievement":
          ui.setToast(`Achievement: ${message.data.name}`);
          session.achievements.add(message.data.id);
          break;
        case "notification":
          ui.setToast(message.data.text);
          break;
        case "npcInteraction":
          ui.setNpcInteraction(message.data);
          break;
        case "questUpdate":
          session.quests = session.quests.filter((q) => q.id !== message.data.quest.id);
          session.quests.push(message.data.quest);
          break;
        default:
          break;
      }
    };

    this.ws.onclose = () => {
      if (this.inputInterval) {
        clearInterval(this.inputInterval);
        this.inputInterval = null;
      }
      world.chunks.clear();
      world.players.clear();
    };
  }

  startInputLoop(sendInput: () => ClientMessage | null) {
    if (this.inputInterval) {
      clearInterval(this.inputInterval);
    }
    this.inputInterval = window.setInterval(() => {
      const msg = sendInput();
      if (msg) {
        this.send(msg);
      }
    }, 50);
  }

  send(message: ClientMessage) {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) return;
    this.ws.send(JSON.stringify(message));
  }
}
