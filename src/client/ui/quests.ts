import { input } from '../input';
import type { PlayerState, Quest } from '../types';

export class QuestManager {
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
    this.modal.id = 'quests-modal';
    this.modal.className = 'modal';
    this.modal.innerHTML = `
      <div class="modal-content">
        <div class="modal-header">
          <h2>Quests</h2>
          <button class="close-btn">&times;</button>
        </div>
        <div class="quests-list" id="quests-list"></div>
      </div>
    `;

    document.body.appendChild(this.modal);
    this.isOpen = true;
    input.registerModal('quests');

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
    input.unregisterModal('quests');
  }

  update(state: PlayerState): void {
    if (this.isOpen) {
      this.render(state);
    }
  }

  private render(state?: PlayerState): void {
    if (!this.modal) return;

    const playerState = state || (window as unknown as { playerState?: PlayerState }).playerState;
    const list = this.modal.querySelector('#quests-list') as HTMLElement;

    if (!playerState || playerState.quests.length === 0) {
      list.innerHTML = '<p class="empty">No active quests</p>';
      return;
    }

    list.innerHTML = playerState.quests.map(quest => this.renderQuest(quest)).join('');
  }

  private renderQuest(quest: Quest): string {
    const stateClass = quest.state.toLowerCase();
    const objectives = quest.objectives?.map(obj => {
      const pct = (obj.current / obj.target) * 100;
      return `
        <div class="objective">
          <span>${obj.description}</span>
          <div class="progress-bar">
            <div class="progress-fill" style="width: ${pct}%"></div>
          </div>
          <span>${obj.current}/${obj.target}</span>
        </div>
      `;
    }).join('') || '';

    return `
      <div class="quest-card ${stateClass}">
        <div class="quest-header">
          <h3>${quest.name}</h3>
          <span class="quest-state">${this.formatState(quest.state)}</span>
        </div>
        ${quest.description ? `<p class="quest-description">${quest.description}</p>` : ''}
        ${objectives ? `<div class="quest-objectives">${objectives}</div>` : ''}
      </div>
    `;
  }

  private formatState(state: string): string {
    switch (state) {
      case 'NotStarted': return 'Not Started';
      case 'InProgress': return 'In Progress';
      case 'ReadyToTurnIn': return 'Ready to Turn In';
      case 'Completed': return 'Completed';
      default: return state;
    }
  }
}

export const quests = new QuestManager();
