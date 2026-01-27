import { EntitySnapshot } from "../state/world";
import { InventoryState } from "../state/inventory";
import { PlayerProfile } from "../state/player";
import { QuestState } from "../state/quests";
import { ALL_ACHIEVEMENTS } from "../data/achievements";
import { CLIENT_RECIPES } from "../data/recipes";
import { drawHud } from "./hud";
import { drawInventory } from "./inventory";
import { drawCrafting } from "./crafting";
import { drawProfile } from "./profile";
import { drawQuestLog } from "./quests";
import { drawAchievements } from "./achievements";
import { drawGameOver, drawSessionRevoked } from "./screens";
import { drawNpcModal, drawToast, NpcModal, Toast } from "./notifications";

export type UiTab = "Inventory" | "Crafting" | "Profile" | "Quests" | "Achievements";

export interface UiRenderState {
  menuOpen: boolean;
  activeTab: UiTab;
  selectedRecipeId: string | null;
  selectedAchievementId: string | null;
  nameBuffer: string;
  loginBuffer: string;
  inventory: InventoryState;
  profile: PlayerProfile;
  quests: QuestState;
}

export interface UiRenderCache {
  menuButton: { x: number; y: number; w: number; h: number };
  recipeBoxes: { id: string; x: number; y: number; w: number; h: number }[];
  craftBox: { x: number; y: number; w: number; h: number };
  profileBoxes: {
    copyBox: { x: number; y: number; w: number; h: number };
    nameBox: { x: number; y: number; w: number; h: number };
    updateBox: { x: number; y: number; w: number; h: number };
    loginBox: { x: number; y: number; w: number; h: number };
    loginButton: { x: number; y: number; w: number; h: number };
  };
  questPins: { id: string; x: number; y: number; w: number; h: number }[];
  achievementBoxes: { id: string; x: number; y: number; w: number; h: number }[];
  achievementPin: { x: number; y: number; w: number; h: number };
  npcOptions: { index: number; x: number; y: number; w: number; h: number }[];
}

export function renderUi(
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number,
  state: UiRenderState,
  player: EntitySnapshot | null,
  toast: Toast | null,
  npcModal: NpcModal | null,
  gameOver: boolean,
  sessionRevoked: boolean
): UiRenderCache {
  drawHud(ctx, width, height, player, state.inventory);
  const menuButton = drawMenuButton(ctx, width);

  let recipeBoxes: UiRenderCache["recipeBoxes"] = [];
  let craftBox = { x: 0, y: 0, w: 0, h: 0 };
  let profileBoxes: UiRenderCache["profileBoxes"] = {
    copyBox: { x: 0, y: 0, w: 0, h: 0 },
    nameBox: { x: 0, y: 0, w: 0, h: 0 },
    updateBox: { x: 0, y: 0, w: 0, h: 0 },
    loginBox: { x: 0, y: 0, w: 0, h: 0 },
    loginButton: { x: 0, y: 0, w: 0, h: 0 },
  };
  let questPins: UiRenderCache["questPins"] = [];
  let achievementBoxes: UiRenderCache["achievementBoxes"] = [];
  let achievementPin = { x: 0, y: 0, w: 0, h: 0 };

  if (state.menuOpen) {
    const menuX = 40;
    const menuY = 60;
    const menuW = width - 80;
    const menuH = height - 120;
    ctx.fillStyle = "rgba(0,0,0,0.7)";
    ctx.fillRect(menuX, menuY, menuW, menuH);

    const tabs: UiTab[] = ["Inventory", "Crafting", "Profile", "Quests", "Achievements"];
    tabs.forEach((tab, idx) => {
      const tabX = menuX + idx * 120;
      ctx.fillStyle = tab === state.activeTab ? "rgba(255,255,255,0.2)" : "rgba(255,255,255,0.05)";
      ctx.fillRect(tabX, menuY, 110, 26);
      ctx.fillStyle = "white";
      ctx.font = "12px sans-serif";
      ctx.textAlign = "center";
      ctx.fillText(tab, tabX + 55, menuY + 18);
    });

    const contentX = menuX + 12;
    const contentY = menuY + 40;
    const contentW = menuW - 24;
    const contentH = menuH - 60;

    if (state.activeTab === "Inventory") {
      drawInventory(ctx, contentX, contentY, contentW, state.inventory);
    }

    if (state.activeTab === "Crafting") {
      const result = drawCrafting(ctx, contentX, contentY, contentW, contentH, CLIENT_RECIPES, state.selectedRecipeId);
      recipeBoxes = result.recipeBoxes;
      craftBox = result.craftBox;
    }

    if (state.activeTab === "Profile") {
      profileBoxes = drawProfile(ctx, contentX, contentY, contentW, contentH, state.profile, state.nameBuffer, state.loginBuffer);
    }

    if (state.activeTab === "Quests") {
      const result = drawQuestLog(ctx, contentX, contentY, contentW, contentH, state.quests);
      questPins = result.pinBoxes;
    }

    if (state.activeTab === "Achievements") {
      const result = drawAchievements(ctx, contentX, contentY, contentW, contentH, ALL_ACHIEVEMENTS, state.selectedAchievementId);
      achievementBoxes = result.selectionBoxes;
      achievementPin = result.pinBox;
    }
  }

  const modal = drawNpcModal(ctx, npcModal, width, height);
  drawToast(ctx, toast, width, height);

  if (gameOver) {
    drawGameOver(ctx, width, height);
  }
  if (sessionRevoked) {
    drawSessionRevoked(ctx, width, height);
  }

  return {
    menuButton,
    recipeBoxes,
    craftBox,
    profileBoxes,
    questPins,
    achievementBoxes,
    achievementPin,
    npcOptions: modal.optionBoxes,
  };
}

function drawMenuButton(ctx: CanvasRenderingContext2D, width: number) {
  const box = { x: width - 44, y: 10, w: 34, h: 24 };
  ctx.fillStyle = "rgba(0,0,0,0.5)";
  ctx.fillRect(box.x, box.y, box.w, box.h);
  ctx.fillStyle = "white";
  ctx.font = "14px sans-serif";
  ctx.textAlign = "center";
  ctx.fillText("MENU", box.x + box.w / 2, box.y + 17);
  return box;
}
