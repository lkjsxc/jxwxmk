import { NetClient } from "../net/ws";
import { InputManager } from "../input/input_manager";
import { InventoryState } from "../state/inventory";
import { PlayerProfile } from "../state/player";
import { QuestState } from "../state/quests";
import { EntitySnapshot } from "../state/world";
import { renderUi, UiRenderCache, UiRenderState, UiTab } from "./ui_render";
import { handleKeyBuffer, handleMenuClick, hotbarHitIndex, hitBox, UiMutableState } from "./ui_helpers";

export class UIManager implements UiMutableState {
  menuOpen = false;
  activeTab: UiTab = "Inventory";
  selectedRecipeId: string | null = null;
  selectedAchievementId: string | null = null;
  toast: { text: string; expiresAt: number } | null = null;
  npcModal: { npcId: string; name: string; text: string; options: string[] } | null = null;
  sessionRevoked = false;
  gameOver = false;
  nameBuffer = "";
  loginBuffer = "";
  editingName = false;
  editingLogin = false;
  pinnedAchievementId: string | null = null;
  cache: UiRenderCache = {
    menuButton: { x: 0, y: 0, w: 0, h: 0 },
    recipeBoxes: [],
    craftBox: { x: 0, y: 0, w: 0, h: 0 },
    profileBoxes: {
      copyBox: { x: 0, y: 0, w: 0, h: 0 },
      nameBox: { x: 0, y: 0, w: 0, h: 0 },
      updateBox: { x: 0, y: 0, w: 0, h: 0 },
      loginBox: { x: 0, y: 0, w: 0, h: 0 },
      loginButton: { x: 0, y: 0, w: 0, h: 0 },
    },
    questPins: [],
    achievementBoxes: [],
    achievementPin: { x: 0, y: 0, w: 0, h: 0 },
    npcOptions: [],
  };

  constructor(
    private net: NetClient,
    private input: InputManager,
    private inventory: InventoryState,
    private profile: PlayerProfile,
    private quests: QuestState
  ) {}

  setToast(toast: { text: string; expiresAt: number }): void {
    this.toast = toast;
  }

  setNpcModal(modal: { npcId: string; name: string; text: string; options: string[] } | null): void {
    this.npcModal = modal;
  }

  setSessionRevoked(): void {
    this.sessionRevoked = true;
  }

  setGameOver(value: boolean): void {
    this.gameOver = value;
  }

  render(ctx: CanvasRenderingContext2D, width: number, height: number, player: EntitySnapshot | null): void {
    const state: UiRenderState = {
      menuOpen: this.menuOpen,
      activeTab: this.activeTab,
      selectedRecipeId: this.selectedRecipeId,
      selectedAchievementId: this.selectedAchievementId,
      nameBuffer: this.nameBuffer,
      loginBuffer: this.loginBuffer,
      inventory: this.inventory,
      profile: this.profile,
      quests: this.quests,
    };
    this.cache = renderUi(
      ctx,
      width,
      height,
      state,
      player,
      this.toast,
      this.npcModal,
      this.gameOver,
      this.sessionRevoked
    );
    handleKeyBuffer(this, this.input);
  }

  handleClick(x: number, y: number, width: number, height: number): void {
    if (this.sessionRevoked) {
      return;
    }
    if (this.gameOver) {
      this.net.send({ type: "spawn", data: { settlement_id: null } });
      this.input.cancelActions();
      return;
    }

    const npcOption = this.cache.npcOptions.find((box) => hitBox(box, x, y));
    if (this.npcModal && npcOption) {
      this.net.send({ type: "npcAction", data: { npc_id: this.npcModal.npcId, option: npcOption.index } });
      this.npcModal = null;
      this.input.cancelActions();
      return;
    }

    if (hitBox(this.cache.menuButton, x, y)) {
      this.menuOpen = !this.menuOpen;
      this.input.cancelActions();
      return;
    }

    if (!this.menuOpen) {
      const slot = hotbarHitIndex(x, y, width, height);
      if (slot !== null) {
        this.inventory.activeSlot = slot;
        this.net.send({ type: "slot", data: { slot } });
      }
      this.input.cancelActions();
      return;
    }

    handleMenuClick(this, this.cache, x, y, width, this.net, this.profile, this.quests, this.input);
  }

  isUiBlocking(): boolean {
    return this.menuOpen || this.npcModal !== null || this.gameOver || this.sessionRevoked;
  }
}
