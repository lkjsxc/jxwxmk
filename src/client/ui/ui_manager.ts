import { PlayerSession } from "../state/player";
import { WorldState } from "../state/world";
import { Camera } from "../rendering/camera";
import { drawHUD } from "./hud";
import { drawInventory } from "./inventory";
import { drawCrafting, RECIPES } from "./crafting";
import { drawProfile } from "./profile";
import { drawQuests } from "./quests";
import { ALL_ACHIEVEMENTS, drawAchievements } from "./achievements";
import { drawGameOver, drawSessionRevoked } from "./screens";
import { drawNpcModal, drawPinnedTracker, drawToast, NpcInteraction, Toast } from "./notifications";

export type MenuTab = "Inventory" | "Crafting" | "Profile" | "Quests" | "Achievements";

export interface UIActions {
  onCraft: (recipe: string) => void;
  onRespawn: () => void;
  onNpcAction: (npcId: string, option: number) => void;
  onNameUpdate: (name: string) => void;
  onLogin: (playerId: string) => void;
  onPinQuest: (questId: string) => void;
}

export class UIManager {
  menuOpen = false;
  activeTab: MenuTab = "Inventory";
  selectedRecipe = 0;
  selectedAchievement: string | null = null;
  toast: Toast | null = null;
  npcInteraction: NpcInteraction | null = null;
  gameOver = false;
  sessionRevoked = false;
  nameBuffer = "";
  loginBuffer = "";
  activeInput: "name" | "login" | null = null;

  private lastCraftLayout: ReturnType<typeof drawCrafting> | null = null;
  private lastProfileLayout: ReturnType<typeof drawProfile> | null = null;
  private lastQuestLayout: ReturnType<typeof drawQuests> | null = null;
  private lastAchievementLayout: ReturnType<typeof drawAchievements> | null = null;
  private lastNpcButtons: Array<{ option: number; x: number; y: number; w: number; h: number }> = [];
  private lastRespawnButton: { x: number; y: number; w: number; h: number } | null = null;

  constructor(private actions: UIActions) {}

  setToast(text: string) {
    this.toast = { text, until: Date.now() + 3000 };
  }

  setNpcInteraction(modal: NpcInteraction | null) {
    this.npcInteraction = modal;
  }

  setGameOver(state: boolean) {
    this.gameOver = state;
  }

  setSessionRevoked(state: boolean) {
    this.sessionRevoked = state;
  }

  handleKeys(keys: string[]) {
    if (!this.menuOpen || this.activeTab !== "Profile") return;
    for (const key of keys) {
      if (key === "\b") {
        if (this.activeInput === "login") {
          this.loginBuffer = this.loginBuffer.slice(0, -1);
        } else {
          this.nameBuffer = this.nameBuffer.slice(0, -1);
        }
      } else {
        if (this.activeInput === "login") {
          this.loginBuffer += key;
        } else {
          this.nameBuffer += key;
        }
      }
    }
  }

  handleClick(x: number, y: number, session: PlayerSession) {
    if (this.sessionRevoked) return;
    if (this.gameOver && this.lastRespawnButton) {
      if (pointInRect(x, y, this.lastRespawnButton)) {
        this.gameOver = false;
        this.actions.onRespawn();
      }
      return;
    }

    if (this.npcInteraction) {
      for (const button of this.lastNpcButtons) {
        if (pointInRect(x, y, button)) {
          this.actions.onNpcAction(this.npcInteraction.npc_id, button.option);
          if (this.npcInteraction.options[button.option] === "Goodbye") {
            this.npcInteraction = null;
          }
          return;
        }
      }
    }

    const toggleRect = { x: window.innerWidth - 60, y: 20, w: 40, h: 24 };
    if (pointInRect(x, y, toggleRect)) {
      this.menuOpen = !this.menuOpen;
      return;
    }

    if (!this.menuOpen) return;
    const tabs: MenuTab[] = ["Inventory", "Crafting", "Profile", "Quests", "Achievements"];
    tabs.forEach((tab, index) => {
      const rect = { x: 100 + index * 120, y: 80, w: 110, h: 26 };
      if (pointInRect(x, y, rect)) {
        this.activeTab = tab;
      }
    });

    if (this.activeTab === "Crafting" && this.lastCraftLayout) {
      this.lastCraftLayout.listRects.forEach((rect, index) => {
        if (pointInRect(x, y, rect)) {
          this.selectedRecipe = index;
        }
      });
      if (pointInRect(x, y, this.lastCraftLayout.craftButton)) {
        this.actions.onCraft(RECIPES[this.selectedRecipe].id);
      }
    }

    if (this.activeTab === "Profile" && this.lastProfileLayout) {
      if (pointInRect(x, y, this.lastProfileLayout.copyButton) && session.id) {
        navigator.clipboard.writeText(session.id).then(() => {
          this.setToast("Player ID copied");
        });
      }
      if (pointInRect(x, y, this.lastProfileLayout.nameField)) {
        this.activeInput = "name";
      }
      if (pointInRect(x, y, this.lastProfileLayout.loginField)) {
        this.activeInput = "login";
      }
      if (pointInRect(x, y, this.lastProfileLayout.nameButton)) {
        this.actions.onNameUpdate(this.nameBuffer.trim());
      }
      if (pointInRect(x, y, this.lastProfileLayout.loginButton)) {
        this.actions.onLogin(this.loginBuffer.trim());
      }
      if (session.id) {
        // noop placeholder for copy button
      }
    }

    if (this.activeTab === "Quests" && this.lastQuestLayout) {
      for (const rect of this.lastQuestLayout.pinButtons) {
        if (pointInRect(x, y, rect)) {
          this.actions.onPinQuest(rect.id);
        }
      }
    }

    if (this.activeTab === "Achievements" && this.lastAchievementLayout) {
      for (const rect of this.lastAchievementLayout.selectRects) {
        if (pointInRect(x, y, rect)) {
          this.selectedAchievement = rect.id;
        }
      }
    }
  }

  render(
    ctx: CanvasRenderingContext2D,
    session: PlayerSession,
    world: WorldState,
    camera: Camera,
  ) {
    const local = session.id ? world.players.get(session.id) : undefined;
    drawHUD(ctx, local);
    drawPinnedTracker(ctx, session.quests.find((q) => q.id === session.pinnedQuestId));
    drawToast(ctx, this.toast);

    this.lastNpcButtons = drawNpcModal(ctx, this.npcInteraction);

    if (this.gameOver) {
      this.lastRespawnButton = drawGameOver(ctx).respawn;
    } else {
      this.lastRespawnButton = null;
    }

    if (this.sessionRevoked) {
      drawSessionRevoked(ctx);
      return;
    }

    if (!this.menuOpen) {
      drawMenuButton(ctx);
      return;
    }

    drawMenuButton(ctx);
    drawTabs(ctx, this.activeTab);
    if (this.activeTab === "Inventory") {
      drawInventory(ctx);
    }
    if (this.activeTab === "Crafting") {
      this.lastCraftLayout = drawCrafting(ctx, this.selectedRecipe);
    }
    if (this.activeTab === "Profile") {
      this.lastProfileLayout = drawProfile(ctx, session, this.nameBuffer, this.loginBuffer);
    }
    if (this.activeTab === "Quests") {
      this.lastQuestLayout = drawQuests(ctx, session.quests);
    }
    if (this.activeTab === "Achievements") {
      this.lastAchievementLayout = drawAchievements(
        ctx,
        session.achievements,
        this.selectedAchievement,
      );
    }
  }
}

function drawMenuButton(ctx: CanvasRenderingContext2D) {
  ctx.save();
  ctx.fillStyle = "rgba(15,23,42,0.8)";
  ctx.fillRect(ctx.canvas.width - 60, 20, 40, 24);
  ctx.fillStyle = "#e2e8f0";
  ctx.font = "12px Space Grotesk";
  ctx.fillText("MENU", ctx.canvas.width - 54, 36);
  ctx.restore();
}

function drawTabs(ctx: CanvasRenderingContext2D, active: MenuTab) {
  const tabs: MenuTab[] = ["Inventory", "Crafting", "Profile", "Quests", "Achievements"];
  tabs.forEach((tab, index) => {
    const x = 100 + index * 120;
    const y = 80;
    ctx.fillStyle = active === tab ? "#1f2937" : "rgba(15,23,42,0.8)";
    ctx.fillRect(x, y, 110, 26);
    ctx.fillStyle = "#e2e8f0";
    ctx.font = "12px Space Grotesk";
    ctx.fillText(tab, x + 10, y + 18);
  });
}

function pointInRect(x: number, y: number, rect: { x: number; y: number; w: number; h: number }) {
  return x >= rect.x && x <= rect.x + rect.w && y >= rect.y && y <= rect.y + rect.h;
}
