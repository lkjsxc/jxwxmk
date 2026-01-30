/**
 * JXWXMK Client Entry Point
 *
 * Connects to the WebSocket server, handles input, rendering, and UI.
 * The client is a "dumb renderer" - all game logic is server-authoritative.
 */

import { connection } from './connection';
import { input } from './input';
import { camera } from './camera';
import { world } from './world';
import { renderer } from './renderer';
import { ui } from './ui/manager';
import { screens } from './ui/screens';
import { notifications } from './ui/notifications';
import type {
  ServerMessage,
  WelcomeMessage,
  PlayerUpdateMessage,
  ChunkAddMessage,
  ChunkRemoveMessage,
  EntityDeltaMessage,
  SessionRevokedMessage,
  AchievementMessage,
  NotificationMessage,
  ErrorMessage,
  QuestUpdateMessage,
} from './types';

// ===== Game State =====

interface GameState {
  playerId: string | null;
  playerName: string;
  spawned: boolean;
  connected: boolean;
}

const state: GameState = {
  playerId: null,
  playerName: '',
  spawned: false,
  connected: false,
};

// ===== Message Handlers =====

function handleWelcome(msg: WelcomeMessage): void {
  state.playerId = msg.id;
  state.spawned = msg.spawned;
  state.connected = true;

  console.log('[Client] Welcome received, player ID:', msg.id);

  if (!msg.spawned) {
    // Send spawn request
    connection.send({ type: 'spawn', data: { settlement_id: null } });
  }

  ui.showScreen('game');
  input.start();
  renderer.start();
}

function handlePlayerUpdate(msg: PlayerUpdateMessage): void {
  const data = msg.data;

  state.playerId = data.id;
  state.playerName = data.name;
  state.spawned = data.spawned;

  // Update world player tracking with position from server
  if (data.spawned) {
    world.setPlayerId(data.id);
    // Update camera to follow player using position from playerUpdate
    camera.follow(data.x, data.y);
  }

  // Update UI
  ui.updatePlayerState(data);
}

function handleChunkAdd(msg: ChunkAddMessage): void {
  world.addChunk(msg.data);
}

function handleChunkRemove(msg: ChunkRemoveMessage): void {
  world.removeChunk(msg.data.coord);
}

function handleEntityDelta(msg: EntityDeltaMessage): void {
  world.applyEntityDelta(msg.data);

  // Check for game over
  if (world.isGameOver()) {
    ui.showScreen('gameover');
  }
}

function handleSessionRevoked(msg: SessionRevokedMessage): void {
  console.log('[Client] Session revoked:', msg.reason);

  state.connected = false;
  state.spawned = false;

  input.stop();
  renderer.stop();
  world.clear();

  ui.showScreen('session_revoked');
}

function handleAchievement(msg: AchievementMessage): void {
  ui.showAchievement(msg);
}

function handleNotification(msg: NotificationMessage): void {
  ui.showNotification(msg);
}

function handleError(msg: ErrorMessage): void {
  console.error('[Client] Server error:', msg.data);
  ui.showError(msg);
}

function handleQuestUpdate(msg: QuestUpdateMessage): void {
  // Quest updates are handled via playerUpdate as well
  // This is just for incremental updates/toasts
  console.log('[Client] Quest update:', msg.data.quest);
}

// ===== Main Message Router =====

function handleMessage(msg: ServerMessage): void {
  switch (msg.type) {
    case 'welcome':
      handleWelcome(msg);
      break;
    case 'playerUpdate':
      handlePlayerUpdate(msg);
      break;
    case 'chunkAdd':
      handleChunkAdd(msg);
      break;
    case 'chunkRemove':
      handleChunkRemove(msg);
      break;
    case 'entityDelta':
      handleEntityDelta(msg);
      break;
    case 'sessionRevoked':
      handleSessionRevoked(msg);
      break;
    case 'achievement':
      handleAchievement(msg);
      break;
    case 'notification':
      handleNotification(msg);
      break;
    case 'error':
      handleError(msg);
      break;
    case 'questUpdate':
      handleQuestUpdate(msg);
      break;
    default:
      console.log('[Client] Unhandled message:', msg);
  }
}

// ===== Initialization =====

function init(): void {
  console.log('[Client] Initializing...');

  // Setup connection handlers
  connection.onMessage(handleMessage);

  connection.onClose((code, reason) => {
    console.log('[Client] Connection closed:', code, reason);
    state.connected = false;
    input.stop();
    renderer.stop();
    world.clear();

    // Show login screen after disconnect
    if (ui.getCurrentScreen() !== 'session_revoked') {
      ui.showScreen('login');
    }
  });

  // Setup zoom callback
  input.emitZoom = (delta) => {
    camera.adjustZoom(delta);
  };

  // Check for existing session
  const storedToken = connection.loadStoredToken();
  if (storedToken) {
    console.log('[Client] Found stored token, auto-connecting...');
    connection.connect();
  }

  // Initial screen
  ui.showScreen('login');

  console.log('[Client] Initialized');
}

// Start when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
