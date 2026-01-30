import { connection } from '../connection';
import type { GameScreen } from '../types';

export class ScreenManager {
  private currentScreen: GameScreen = 'login';
  private loginScreen: HTMLElement;
  private hud: HTMLElement;

  constructor() {
    this.loginScreen = document.getElementById('login-screen')!;
    this.hud = document.getElementById('hud')!;

    this.setupLoginScreen();
  }

  private setupLoginScreen(): void {
    const connectBtn = document.getElementById('connect-btn')!;
    const playerIdInput = document.getElementById('player-id') as HTMLInputElement;

    connectBtn.addEventListener('click', () => {
      const playerId = playerIdInput.value.trim();
      this.doConnect(playerId || null);
    });

    playerIdInput.addEventListener('keypress', (e) => {
      if (e.key === 'Enter') {
        const playerId = playerIdInput.value.trim();
        this.doConnect(playerId || null);
      }
    });
  }

  private async doConnect(playerId: string | null): Promise<void> {
    if (playerId) {
      // Claim session for existing player
      try {
        const response = await fetch('/session/claim', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ player_id: playerId }),
        });

        if (!response.ok) {
          const error = await response.json();
          alert(`Failed to claim session: ${error.message || 'Unknown error'}`);
          return;
        }

        const data = await response.json();
        connection.setSessionToken(data.token);
      } catch (err) {
        console.error('Failed to claim session:', err);
        alert('Failed to connect to server');
        return;
      }
    } else {
      // New player - will get token from welcome message
      connection.clearToken();
    }

    connection.connect();
  }

  show(screen: GameScreen): void {
    this.currentScreen = screen;

    // Hide all screens
    this.loginScreen.classList.add('hidden');
    this.hud.classList.add('hidden');

    // Show requested screen
    switch (screen) {
      case 'login':
        this.loginScreen.classList.remove('hidden');
        break;
      case 'game':
        this.hud.classList.remove('hidden');
        break;
      case 'gameover':
        this.showGameOver();
        break;
      case 'session_revoked':
        this.showSessionRevoked();
        break;
    }
  }

  private showGameOver(): void {
    // Create game over overlay
    let overlay = document.getElementById('gameover-overlay');
    if (!overlay) {
      overlay = document.createElement('div');
      overlay.id = 'gameover-overlay';
      overlay.className = 'overlay';
      overlay.innerHTML = `
        <div class="overlay-content">
          <h1>YOU DIED</h1>
          <button id="respawn-btn">Respawn</button>
        </div>
      `;
      document.body.appendChild(overlay);

      const respawnBtn = document.getElementById('respawn-btn')!;
      respawnBtn.addEventListener('click', () => {
        connection.send({ type: 'spawn', data: { settlement_id: null } });
        overlay!.remove();
      });
    }
  }

  private showSessionRevoked(): void {
    connection.clearToken();

    let overlay = document.getElementById('revoked-overlay');
    if (!overlay) {
      overlay = document.createElement('div');
      overlay.id = 'revoked-overlay';
      overlay.className = 'overlay';
      overlay.innerHTML = `
        <div class="overlay-content">
          <h1>Session Revoked</h1>
          <p>Your session was logged in elsewhere.</p>
          <button id="reconnect-btn">Reconnect</button>
        </div>
      `;
      document.body.appendChild(overlay);

      const reconnectBtn = document.getElementById('reconnect-btn')!;
      reconnectBtn.addEventListener('click', () => {
        overlay!.remove();
        this.show('login');
      });
    }
  }

  getCurrentScreen(): GameScreen {
    return this.currentScreen;
  }
}

export const screens = new ScreenManager();
