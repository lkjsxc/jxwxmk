import type { PlayerState, InventorySlot } from '../types';

export class InventoryManager {
  private modal: HTMLElement | null;
  private grid: HTMLElement | null;
  private isOpen = false;

  // Stats elements
  private levelEl: HTMLElement | null;
  private xpEl: HTMLElement | null;
  private killsEl: HTMLElement | null;
  private craftsEl: HTMLElement | null;
  private deathsEl: HTMLElement | null;

  constructor() {
    this.modal = document.getElementById('inventory-modal');
    this.grid = document.getElementById('inventory-grid');
    this.levelEl = document.getElementById('inv-level');
    this.xpEl = document.getElementById('inv-xp');
    this.killsEl = document.getElementById('inv-kills');
    this.craftsEl = document.getElementById('inv-crafts');
    this.deathsEl = document.getElementById('inv-deaths');

    this.setupListeners();
  }

  private setupListeners(): void {
    document.getElementById('close-inventory')?.addEventListener('click', () => {
      this.close();
    });
  }

  toggle(): void {
    if (this.isOpen) {
      this.close();
    } else {
      this.open();
    }
  }

  open(): void {
    this.modal?.classList.remove('hidden');
    this.isOpen = true;
  }

  close(): void {
    this.modal?.classList.add('hidden');
    this.isOpen = false;
  }

  isOpened(): boolean {
    return this.isOpen;
  }

  render(playerState: PlayerState): void {
    if (!this.grid) return;

    this.grid.innerHTML = '';

    playerState.inventory.forEach((slot, index) => {
      const slotEl = document.createElement('div');
      const isActive = index === playerState.active_slot;
      slotEl.className = `inv-slot ${isActive ? 'active' : ''}`;

      if (slot) {
        slotEl.innerHTML = `
          <div class="item-icon">${slot.item.charAt(0).toUpperCase()}</div>
          <div class="item-count">${slot.count}</div>
          <div class="item-name">${slot.item}</div>
        `;
      } else {
        slotEl.classList.add('empty');
      }

      slotEl.addEventListener('click', () => {
        this.onSlotSelect?.(index);
      });

      this.grid.appendChild(slotEl);
    });

    // Update stats
    if (this.levelEl) this.levelEl.textContent = playerState.level.toString();
    if (this.xpEl) this.xpEl.textContent = playerState.xp.toString();
    if (this.killsEl) this.killsEl.textContent = playerState.stats.kills.toString();
    if (this.craftsEl) this.craftsEl.textContent = playerState.stats.crafts.toString();
    if (this.deathsEl) this.deathsEl.textContent = playerState.stats.deaths.toString();
  }

  // Callback for slot selection
  onSlotSelect: ((slot: number) => void) | null = null;
}
