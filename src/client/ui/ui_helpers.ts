import { InputManager } from "../input/input_manager";
import { NetClient } from "../net/ws";
import { PlayerProfile } from "../state/player";
import { QuestState } from "../state/quests";
import { UiRenderCache, UiTab } from "./ui_render";

export interface UiMutableState {
  activeTab: UiTab;
  selectedRecipeId: string | null;
  selectedAchievementId: string | null;
  nameBuffer: string;
  loginBuffer: string;
  editingName: boolean;
  editingLogin: boolean;
  toast: { text: string; expiresAt: number } | null;
  pinnedAchievementId: string | null;
}

export function handleMenuClick(
  state: UiMutableState,
  cache: UiRenderCache,
  x: number,
  y: number,
  width: number,
  net: NetClient,
  profile: PlayerProfile,
  quests: QuestState,
  input: InputManager
): void {
  const menuX = 40;
  const menuY = 60;
  const tabs: UiTab[] = ["Inventory", "Crafting", "Profile", "Quests", "Achievements"];
  tabs.forEach((tab, idx) => {
    const tabBox = { x: menuX + idx * 120, y: menuY, w: 110, h: 26 };
    if (hitBox(tabBox, x, y)) {
      state.activeTab = tab;
    }
  });

  if (state.activeTab === "Crafting") {
    const recipe = cache.recipeBoxes.find((box) => hitBox(box, x, y));
    if (recipe) {
      state.selectedRecipeId = recipe.id;
    }
    if (hitBox(cache.craftBox, x, y) && state.selectedRecipeId) {
      net.send({ type: "craft", data: { recipe: state.selectedRecipeId } });
    }
  }

  if (state.activeTab === "Profile") {
    const { copyBox, nameBox, updateBox, loginBox, loginButton } = cache.profileBoxes;
    if (hitBox(copyBox, x, y) && profile.id) {
      navigator.clipboard.writeText(profile.id).catch(() => undefined);
      state.toast = { text: "Player ID copied", expiresAt: Date.now() + 2000 };
    }
    if (hitBox(nameBox, x, y)) {
      state.editingName = true;
      state.editingLogin = false;
    }
    if (hitBox(loginBox, x, y)) {
      state.editingLogin = true;
      state.editingName = false;
    }
    if (hitBox(updateBox, x, y)) {
      net.send({ type: "name", data: { name: state.nameBuffer || "Traveler" } });
    }
    if (hitBox(loginButton, x, y)) {
      triggerDeviceLogin(state, net);
    }
  }

  if (state.activeTab === "Quests") {
    const pin = cache.questPins.find((box) => hitBox(box, x, y));
    if (pin) {
      quests.pin(pin.id);
    }
  }

  if (state.activeTab === "Achievements") {
    const selection = cache.achievementBoxes.find((box) => hitBox(box, x, y));
    if (selection) {
      state.selectedAchievementId = selection.id;
    }
    if (hitBox(cache.achievementPin, x, y) && state.selectedAchievementId) {
      state.pinnedAchievementId = state.selectedAchievementId;
    }
  }

  input.cancelActions();
}

export function handleKeyBuffer(state: UiMutableState, input: InputManager): void {
  if (!state.editingName && !state.editingLogin) {
    return;
  }
  while (input.keyQueue.length > 0) {
    const key = input.keyQueue.shift();
    if (!key) {
      continue;
    }
    if (key === "\b") {
      if (state.editingName) {
        state.nameBuffer = state.nameBuffer.slice(0, -1);
      } else if (state.editingLogin) {
        state.loginBuffer = state.loginBuffer.slice(0, -1);
      }
      continue;
    }
    if (state.editingName) {
      state.nameBuffer += key;
    } else if (state.editingLogin) {
      state.loginBuffer += key;
    }
  }
}

export function hotbarHitIndex(x: number, y: number, width: number, height: number): number | null {
  const slots = 7;
  const slotSize = 36;
  const gap = 6;
  const totalWidth = slots * slotSize + (slots - 1) * gap;
  const startX = (width - totalWidth) / 2;
  const startY = height - 60;
  for (let i = 0; i < slots; i += 1) {
    const slotX = startX + i * (slotSize + gap);
    const box = { x: slotX, y: startY, w: slotSize, h: slotSize };
    if (hitBox(box, x, y)) {
      return i;
    }
  }
  return null;
}

function triggerDeviceLogin(state: UiMutableState, net: NetClient): void {
  if (state.loginBuffer.trim().length === 0) {
    return;
  }
  fetch("/session/claim", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ player_id: state.loginBuffer.trim() }),
  })
    .then((res) => (res.ok ? res.json() : Promise.reject()))
    .then((payload) => {
      localStorage.setItem("jxwxmk_token", payload.token);
      net.connect(payload.token);
      window.location.reload();
    })
    .catch(() => {
      state.toast = { text: "Invalid Player ID", expiresAt: Date.now() + 2000 };
    });
}

export function hitBox(box: { x: number; y: number; w: number; h: number }, x: number, y: number): boolean {
  return x >= box.x && x <= box.x + box.w && y >= box.y && y <= box.y + box.h;
}
