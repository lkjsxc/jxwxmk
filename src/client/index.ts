import { InputManager } from "./input/input_manager";
import { SocketClient } from "./net/socket";
import { WorldState } from "./state/world";
import { PlayerSession } from "./state/player";
import { UIManager } from "./ui/ui_manager";
import { startRenderLoop } from "./rendering/canvas";

const canvas = document.getElementById("game") as HTMLCanvasElement | null;
if (!canvas) {
  throw new Error("Missing canvas");
}

const world = new WorldState();
const session = new PlayerSession();
const input = new InputManager(canvas);
const socket = new SocketClient();

const ui = new UIManager({
  onCraft: (recipe) => socket.send({ type: "craft", data: { recipe } }),
  onRespawn: () => socket.send({ type: "spawn", data: { settlement_id: null } }),
  onNpcAction: (npcId, option) => socket.send({ type: "npcAction", data: { npc_id: npcId, option } }),
  onNameUpdate: (name) => socket.send({ type: "name", data: { name } }),
  onLogin: async (playerId) => {
    if (!playerId) return;
    const res = await fetch("/session/claim", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ player_id: playerId }),
    });
    if (!res.ok) {
      ui.setToast("Invalid Player ID");
      return;
    }
    const data = await res.json();
    localStorage.setItem("session_token", data.token);
    window.location.reload();
  },
  onPinQuest: (questId) => {
    session.pinnedQuestId = questId;
  },
});

socket.connect(session, world, ui);

socket.startInputLoop(() => {
  input.update();
  if (ui.menuOpen || ui.sessionRevoked || ui.gameOver) {
    return null;
  }
  const { dx, dy, attack, interact } = input.state;
  const hasInput = Math.abs(dx) > 0 || Math.abs(dy) > 0 || attack || interact;
  if (!hasInput) return null;
  return { type: "input", data: { dx, dy, attack, interact } };
});

canvas.addEventListener("click", (event) => {
  ui.handleClick(event.clientX, event.clientY, session);
});

window.addEventListener("keydown", () => {
  const keys = input.consumeKeyQueue();
  ui.handleKeys(keys);
});

startRenderLoop(canvas, world, session, ui);
