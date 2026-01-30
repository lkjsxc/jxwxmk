import { input } from '../input';
import { ALL_ACHIEVEMENTS } from '../types';
import type { PlayerState } from '../types';

export class AchievementsManager {
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
    this.modal.id = 'achievements-modal';
    this.modal.className = 'modal';
    this.modal.innerHTML = `
      <div class="modal-content">
        <div class="modal-header">
          <h2>Achievements</h2>
          <button class="close-btn">&times;</button>
        </div>
        <div class="achievements-list" id="achievements-list"></div>
      </div>
    `;

    document.body.appendChild(this.modal);
    this.isOpen = true;
    input.registerModal('achievements');

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

    this.render();
  }

  close(): void {
    if (this.modal) {
      this.modal.remove();
      this.modal = null;
    }
    this.isOpen = false;
    input.unregisterModal('achievements');
  }

  update(state: PlayerState): void {
    if (this.isOpen) {
      this.render(state);
    }
  }

  private render(state?: PlayerState): void {
    if (!this.modal) return;

    const playerState = state || (window as unknown as { playerState?: PlayerState }).playerState;
    const list = this.modal.querySelector('#achievements-list') as HTMLElement;

    const unlocked = new Set(playerState?.achievements || []);

    list.innerHTML = ALL_ACHIEVEMENTS.map(ach => {
      const isUnlocked = unlocked.has(ach.id);
      return `
        <div class="achievement-item ${isUnlocked ? 'unlocked' : 'locked'}">
          <div class="achievement-icon">${isUnlocked ? '🏆' : '🔒'}</div>
          <div class="achievement-info">
            <h4>${ach.name}</h4>
            <p>${isUnlocked ? 'Unlocked!' : 'Locked'}</p>
          </div>
        </div>
      `;
    }).join('');
  }

  showToast(name: string): void {
    // Toast is handled by notification manager
    import('./notifications').then(({ notifications }) => {
      notifications.show(`Achievement unlocked: ${name}`, 'achievement');
    });
  }
}

export const achievements = new AchievementsManager();
