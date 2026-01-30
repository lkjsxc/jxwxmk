import { screens } from './screens';
import { hud } from './hud';
import { hotbar } from './hotbar';
import { inventory } from './inventory';
import { crafting } from './crafting';
import { quests } from './quests';
import { achievements } from './achievements';
import { profile } from './profile';
import { notifications } from './notifications';
import type { PlayerState, GameScreen, NotificationMessage, AchievementMessage, ErrorMessage } from '../types';

export class UIManager {
  private playerState: PlayerState | null = null;
  private currentScreen: GameScreen = 'login';

  constructor() {
    this.setupKeyboardShortcuts();
    this.addStyles();
  }

  private setupKeyboardShortcuts(): void {
    window.addEventListener('keydown', (e) => {
      // Don't handle keys if typing in an input
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
        return;
      }

      // I = Inventory
      if (e.key.toLowerCase() === 'i') {
        inventory.toggle();
      }

      // C = Crafting
      if (e.key.toLowerCase() === 'c') {
        crafting.toggle();
      }

      // Q = Quests
      if (e.key.toLowerCase() === 'q') {
        quests.toggle();
      }

      // P = Profile
      if (e.key.toLowerCase() === 'p') {
        profile.toggle();
      }

      // Escape = close all modals
      if (e.key === 'Escape') {
        inventory.close();
        crafting.close();
        quests.close();
        achievements.close();
        profile.close();
      }
    });

    // Menu button (top right)
    const menuBtn = document.getElementById('menu-btn');
    if (menuBtn) {
      menuBtn.addEventListener('click', () => {
        profile.toggle();
      });
    }

    // Touch action buttons
    const invBtn = document.getElementById('inv-btn');
    if (invBtn) {
      invBtn.addEventListener('click', () => inventory.toggle());
    }

    const craftBtn = document.getElementById('craft-btn');
    if (craftBtn) {
      craftBtn.addEventListener('click', () => crafting.toggle());
    }

    const questBtn = document.getElementById('quest-btn');
    if (questBtn) {
      questBtn.addEventListener('click', () => quests.toggle());
    }
  }

  private addStyles(): void {
    const style = document.createElement('style');
    style.textContent = `
      /* Modal base styles */
      .modal {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.7);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
      }

      .modal-content {
        background: #1a1a2e;
        border: 2px solid #4a4a6a;
        border-radius: 10px;
        min-width: 300px;
        max-width: 90vw;
        max-height: 80vh;
        overflow: auto;
      }

      .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 15px 20px;
        border-bottom: 1px solid #4a4a6a;
      }

      .modal-header h2 {
        margin: 0;
        font-size: 20px;
      }

      .close-btn {
        background: none;
        border: none;
        color: #fff;
        font-size: 24px;
        cursor: pointer;
      }

      /* Inventory */
      .inventory-grid {
        display: grid;
        gap: 5px;
        padding: 15px;
        max-width: 500px;
      }

      .inventory-slot {
        aspect-ratio: 1;
        background: #2a2a3e;
        border: 2px solid #4a4a6a;
        border-radius: 5px;
        display: flex;
        align-items: center;
        justify-content: center;
        position: relative;
        cursor: pointer;
        font-size: 20px;
      }

      .inventory-slot:hover {
        border-color: #6a6aff;
      }

      .item-count {
        position: absolute;
        bottom: 2px;
        right: 4px;
        font-size: 10px;
        color: #fff;
      }

      /* Crafting */
      .crafting-content {
        min-width: 500px;
      }

      .crafting-body {
        display: flex;
        min-height: 300px;
      }

      .recipe-list {
        width: 40%;
        border-right: 1px solid #4a4a6a;
        overflow-y: auto;
      }

      .recipe-item {
        padding: 10px 15px;
        cursor: pointer;
        border-bottom: 1px solid #2a2a3e;
      }

      .recipe-item:hover {
        background: #2a2a3e;
      }

      .recipe-item.selected {
        background: #3a3a5e;
      }

      .recipe-item.disabled {
        opacity: 0.5;
      }

      .recipe-details {
        flex: 1;
        padding: 20px;
      }

      .requirements {
        margin: 15px 0;
      }

      .requirement {
        margin: 5px 0;
        color: #ff6666;
      }

      .requirement.met {
        color: #66ff66;
      }

      .craft-btn {
        padding: 10px 20px;
        background: #6a6aff;
        color: #fff;
        border: none;
        border-radius: 5px;
        cursor: pointer;
      }

      .craft-btn:disabled {
        background: #4a4a6a;
        cursor: not-allowed;
      }

      /* Quests */
      .quests-list {
        padding: 15px;
      }

      .empty {
        text-align: center;
        color: #888;
        padding: 30px;
      }

      .quest-card {
        background: #2a2a3e;
        border: 1px solid #4a4a6a;
        border-radius: 5px;
        padding: 15px;
        margin-bottom: 10px;
      }

      .quest-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 10px;
      }

      .quest-header h3 {
        margin: 0;
        font-size: 16px;
      }

      .quest-state {
        font-size: 12px;
        padding: 3px 8px;
        background: #4a4a6a;
        border-radius: 3px;
      }

      .quest-card.completed .quest-state {
        background: #2a5a2a;
      }

      .quest-objectives {
        margin-top: 10px;
      }

      .objective {
        display: flex;
        align-items: center;
        gap: 10px;
        margin: 5px 0;
        font-size: 12px;
      }

      .progress-bar {
        flex: 1;
        height: 6px;
        background: #333;
        border-radius: 3px;
        overflow: hidden;
      }

      .progress-fill {
        height: 100%;
        background: #6a6aff;
      }

      /* Achievements */
      .achievements-list {
        padding: 15px;
      }

      .achievement-item {
        display: flex;
        align-items: center;
        gap: 15px;
        padding: 15px;
        border-bottom: 1px solid #2a2a3e;
      }

      .achievement-icon {
        font-size: 24px;
      }

      .achievement-info h4 {
        margin: 0 0 5px 0;
      }

      .achievement-info p {
        margin: 0;
        font-size: 12px;
        color: #888;
      }

      .achievement-item.unlocked .achievement-info p {
        color: #6a6aff;
      }

      /* Profile */
      .profile-body {
        padding: 20px;
      }

      .profile-section {
        margin-bottom: 20px;
      }

      .profile-section h3 {
        margin: 0 0 10px 0;
        font-size: 14px;
        color: #888;
      }

      .player-id-row {
        display: flex;
        gap: 10px;
        align-items: center;
      }

      .player-id-row code {
        background: #2a2a3e;
        padding: 8px 12px;
        border-radius: 5px;
        font-family: monospace;
        font-size: 12px;
        flex: 1;
        overflow: hidden;
        text-overflow: ellipsis;
      }

      .stats-grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 10px;
      }

      .stat-item {
        background: #2a2a3e;
        padding: 10px;
        border-radius: 5px;
        text-align: center;
      }

      .stat-label {
        display: block;
        font-size: 11px;
        color: #888;
        margin-bottom: 5px;
      }

      .stat-value {
        font-size: 18px;
        font-weight: bold;
      }

      .name-row, .device-login-row {
        display: flex;
        gap: 10px;
      }

      .name-row input, .device-login-row input {
        flex: 1;
        padding: 8px 12px;
        background: #2a2a3e;
        border: 1px solid #4a4a6a;
        border-radius: 5px;
        color: #fff;
      }

      .name-row button, .device-login-row button, .player-id-row button {
        padding: 8px 16px;
        background: #6a6aff;
        color: #fff;
        border: none;
        border-radius: 5px;
        cursor: pointer;
      }

      /* Overlays */
      .overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.9);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 2000;
      }

      .overlay-content {
        text-align: center;
      }

      .overlay-content h1 {
        font-size: 48px;
        color: #ff4444;
        margin-bottom: 20px;
      }

      .overlay-content button {
        padding: 15px 30px;
        font-size: 18px;
        background: #6a6aff;
        color: #fff;
        border: none;
        border-radius: 5px;
        cursor: pointer;
      }
    `;
    document.head.appendChild(style);
  }

  // Update all UI components with new player state
  updatePlayerState(state: PlayerState): void {
    this.playerState = state;

    // Store for UI components to access
    (window as unknown as { playerState: PlayerState }).playerState = state;

    hud.update(state);
    hotbar.update(state);
    inventory.update(state);
    crafting.update(state);
    quests.update(state);
    achievements.update(state);
    profile.update(state);
  }

  // Show notification
  showNotification(msg: NotificationMessage): void {
    notifications.show(msg.data.text, 'info');
  }

  // Show achievement
  showAchievement(msg: AchievementMessage): void {
    achievements.showToast(msg.data.name);
  }

  // Show error
  showError(msg: ErrorMessage): void {
    notifications.show(msg.data.message, 'error');
  }

  // Change game screen
  showScreen(screen: GameScreen): void {
    this.currentScreen = screen;
    screens.show(screen);

    if (screen === 'game') {
      hud.show();
    } else {
      hud.hide();
    }
  }

  getPlayerState(): PlayerState | null {
    return this.playerState;
  }

  getCurrentScreen(): GameScreen {
    return this.currentScreen;
  }
}

export const ui = new UIManager();
