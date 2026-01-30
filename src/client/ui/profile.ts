import { connection } from '../connection';
import { input } from '../input';
import type { PlayerState } from '../types';

export class ProfileManager {
  private modal: HTMLElement | null = null;
  private isOpen = false;

  toggle(): void {
    if (this.isOpen) {
      this.close();
    } else {
      this.open();
    }
  }

  open(): void {
    if (this.modal) return;

    this.modal = document.createElement('div');
    this.modal.id = 'profile-modal';
    this.modal.className = 'modal';
    this.modal.innerHTML = `
      <div class="modal-content">
        <div class="modal-header">
          <h2>Profile</h2>
          <button class="close-btn">&times;</button>
        </div>
        <div class="profile-body" id="profile-body">
          <div class="profile-section">
            <h3>Player ID</h3>
            <div class="player-id-row">
              <code id="player-id-display">-</code>
              <button id="copy-id-btn">Copy</button>
            </div>
          </div>
          <div class="profile-section">
            <h3>Progression</h3>
            <div class="progression-info">
              <div>Level: <span id="profile-level">-</span></div>
              <div>XP: <span id="profile-xp">-</span></div>
            </div>
          </div>
          <div class="profile-section">
            <h3>Stats</h3>
            <div class="stats-grid" id="stats-grid"></div>
          </div>
          <div class="profile-section">
            <h3>Name</h3>
            <div class="name-row">
              <input type="text" id="name-input" maxlength="20" placeholder="Enter name">
              <button id="update-name-btn">Update</button>
            </div>
          </div>
          <div class="profile-section">
            <h3>Device Login</h3>
            <div class="device-login-row">
              <input type="text" id="device-login-input" placeholder="Enter Player ID">
              <button id="device-login-btn">Login</button>
            </div>
          </div>
        </div>
      </div>
    `;

    document.body.appendChild(this.modal);
    this.isOpen = true;
    input.registerModal('profile');

    this.modal.querySelector('.close-btn')!.addEventListener('click', () => this.close());
    this.modal.addEventListener('click', (e) => {
      if (e.target === this.modal) this.close();
    });

    const escHandler = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        this.close();
        window.removeEventListener('keydown', escHandler);
      }
    };
    window.addEventListener('keydown', escHandler);

    // Setup button handlers
    this.modal.querySelector('#copy-id-btn')!.addEventListener('click', () => this.copyPlayerId());
    this.modal.querySelector('#update-name-btn')!.addEventListener('click', () => this.updateName());
    this.modal.querySelector('#device-login-btn')!.addEventListener('click', () => this.deviceLogin());

    this.render();
  }

  close(): void {
    if (this.modal) {
      this.modal.remove();
      this.modal = null;
    }
    this.isOpen = false;
    input.unregisterModal('profile');
  }

  update(state: PlayerState): void {
    if (this.isOpen) {
      this.render(state);
    }
  }

  private render(state?: PlayerState): void {
    if (!this.modal) return;

    const playerState = state || (window as unknown as { playerState?: PlayerState }).playerState;
    if (!playerState) return;

    // Update player ID
    const idDisplay = this.modal.querySelector('#player-id-display') as HTMLElement;
    idDisplay.textContent = playerState.id;

    // Update progression
    const levelEl = this.modal.querySelector('#profile-level') as HTMLElement;
    const xpEl = this.modal.querySelector('#profile-xp') as HTMLElement;
    levelEl.textContent = String(playerState.level);
    xpEl.textContent = String(playerState.xp);

    // Update stats
    const statsGrid = this.modal.querySelector('#stats-grid') as HTMLElement;
    const stats = [
      { label: 'Steps', value: playerState.stats.steps },
      { label: 'Kills', value: playerState.stats.kills },
      { label: 'Crafts', value: playerState.stats.crafts },
      { label: 'Gathers', value: playerState.stats.gathers },
      { label: 'Deaths', value: playerState.stats.deaths },
    ];

    statsGrid.innerHTML = stats.map(s => `
      <div class="stat-item">
        <span class="stat-label">${s.label}</span>
        <span class="stat-value">${s.value}</span>
      </div>
    `).join('');

    // Update name input
    const nameInput = this.modal.querySelector('#name-input') as HTMLInputElement;
    nameInput.value = playerState.name;
  }

  private copyPlayerId(): void {
    const playerState = (window as unknown as { playerState?: PlayerState }).playerState;
    if (playerState?.id) {
      navigator.clipboard.writeText(playerState.id).then(() => {
        import('./notifications').then(({ notifications }) => {
          notifications.show('Player ID copied to clipboard');
        });
      });
    }
  }

  private updateName(): void {
    const nameInput = this.modal?.querySelector('#name-input') as HTMLInputElement;
    if (!nameInput) return;

    const name = nameInput.value.trim();
    if (name) {
      connection.send({ type: 'name', data: { name } });
    }
  }

  private async deviceLogin(): Promise<void> {
    const input = this.modal?.querySelector('#device-login-input') as HTMLInputElement;
    if (!input) return;

    const playerId = input.value.trim();
    if (!playerId) return;

    try {
      const response = await fetch('/session/claim', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ player_id: playerId }),
      });

      if (!response.ok) {
        const error = await response.json();
        import('./notifications').then(({ notifications }) => {
          notifications.show(`Login failed: ${error.message || 'Unknown error'}`, 'error');
        });
        return;
      }

      const data = await response.json();
      connection.setSessionToken(data.token);
      connection.disconnect();
      connection.connect();

      this.close();
      import('./notifications').then(({ notifications }) => {
        notifications.show('Logged in successfully');
      });
    } catch (err) {
      import('./notifications').then(({ notifications }) => {
        notifications.show('Failed to connect to server', 'error');
      });
    }
  }
}

export const profile = new ProfileManager();
