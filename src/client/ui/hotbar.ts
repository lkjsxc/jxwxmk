import { connection } from '../connection';
import type { PlayerState } from '../types';

export class HotbarManager {
  private container: HTMLElement;
  private slots: HTMLElement[] = [];
  private activeSlot = 0;

  constructor() {
    this.container = document.getElementById('hotbar')!;
    this.createSlots();
  }

  private createSlots(): void {
    this.container.innerHTML = '';
    this.slots = [];

    for (let i = 0; i < 7; i++) {
      const slot = document.createElement('div');
      slot.className = 'hotbar-slot';
      slot.dataset.slot = String(i);
      slot.title = `Slot ${i + 1} (Press ${i + 1})`;

      slot.addEventListener('click', () => {
        this.selectSlot(i);
      });

      this.container.appendChild(slot);
      this.slots.push(slot);
    }
  }

  update(state: PlayerState): void {
    this.activeSlot = state.active_slot;

    for (let i = 0; i < 7; i++) {
      const slotEl = this.slots[i];
      const item = state.inventory[i];

      // Update active state
      if (i === this.activeSlot) {
        slotEl.classList.add('active');
      } else {
        slotEl.classList.remove('active');
      }

      // Update content
      if (item) {
        slotEl.innerHTML = `
          <div class="item-icon">${this.getItemIcon(item.item)}</div>
          <div class="item-count">${item.count > 1 ? item.count : ''}</div>
        `;
        slotEl.title = `${item.item} x${item.count}`;
      } else {
        slotEl.innerHTML = '';
        slotEl.title = `Slot ${i + 1}`;
      }
    }
  }

  selectSlot(slot: number): void {
    if (slot < 0 || slot >= 7) return;
    connection.send({ type: 'slot', data: { slot } });
  }

  private getItemIcon(itemId: string): string {
    // Simple emoji/text icons
    const icons: Record<string, string> = {
      wood: '🪵',
      stone: '🪨',
      wood_pickaxe: '⛏️',
      stone_pickaxe: '⚒️',
      wood_wall: '🧱',
      door: '🚪',
      torch: '🔥',
      workbench: '🔨',
      berry: '🍒',
      meat: '🍖',
    };
    return icons[itemId] || '📦';
  }
}

export const hotbar = new HotbarManager();
