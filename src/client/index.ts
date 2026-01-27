import { InputManager } from "./input/input_manager";
import { NetClient } from "./net/ws";
import { Renderer } from "./rendering/canvas";
import { createEmptyInventory } from "./state/inventory";
import { createDefaultProfile, loadToken } from "./state/player";
import { QuestState } from "./state/quests";
import { WorldState } from "./state/world";
import { UIManager } from "./ui/ui_manager";

const canvas = document.getElementById("game") as HTMLCanvasElement;
const world = new WorldState();
const quests = new QuestState();
const inventory = createEmptyInventory(30);
const profile = createDefaultProfile();
const input = new InputManager();
const net = new NetClient(world, quests);
const ui = new UIManager(net, input, inventory, profile, quests);
const renderer = new Renderer(canvas, world, ui);

input.attach(canvas);

net.onToast = (toast) => ui.setToast(toast);
net.onNpcModal = (modal) => ui.setNpcModal(modal);
net.onSessionRevoked = () => ui.setSessionRevoked();
net.onWelcome = (id, token) => {
  profile.id = id;
  profile.token = token;
};

net.connect(loadToken());

canvas.addEventListener("click", (event) => {
  ui.handleClick(event.clientX, event.clientY, canvas.width, canvas.height);
});

setInterval(() => {
  input.update();
  const state = input.consumeState();
  if (ui.isUiBlocking()) {
    state.attack = false;
    state.interact = false;
  }
  net.sendInput(state);
}, 50);

function loop(): void {
  ui.setGameOver(world.isGameOver());
  renderer.render();
  requestAnimationFrame(loop);
}

loop();
