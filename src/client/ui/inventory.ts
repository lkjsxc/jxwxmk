import { connection } from '../connection';
import { input } from '../input';
import type { PlayerState, ItemSlot } from '../types';

const INVENTORY_SIZE = 30;

export class InventoryManager {
  private modal: HTMLElement | null = null;
  private isOpen = false;
  private dragFromSlot: number | null = null;

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
    this.modal.id = 'inventory-modal';
    this.modal.className = 'modal';
    this.modal.innerHTML = `
      <div class="modal-content">
        <div class="modal-header">
          <h2>Inventory</h2>
          <button class="close-btn">&times;</button>
        </div>
        <div class="inventory-grid" id="inventory-grid"></div>
      </div>
    `;

    document.body.appendChild(this.modal);
    this.isOpen = true;
    input.registerModal('inventory');

    // Close button
    this.modal.querySelector('.close-btn')!.addEventListener('click', () => this.close());

    // Close on backdrop click
    this.modal.addEventListener('click', (e) => {
      if (e.target === this.modal) this.close();
    });

    // Close on Escape
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
    this.dragFromSlot = null;
    input.unregisterModal('inventory');
  }

  update(state: PlayerState): void {
    if (this.isOpen) {
      this.render(state);
    }
  }

  private render(state?: PlayerState): void {
    if (!this.modal) return;

    // Get current player state from window if not provided
    const playerState = state || (window as unknown as { playerState?: PlayerState }).playerState;
    if (!playerState) return;

    const grid = this.modal.querySelector('#inventory-grid') as HTMLElement;
    grid.innerHTML = '';

    // Calculate columns based on screen width
    const cols = window.innerWidth > 600 ? 7 : window.innerWidth > 400 ? 5 : 3;
    grid.style.gridTemplateColumns = `repeat(${cols}, 1fr)`;

    for (let i = 0; i < INVENTORY_SIZE; i++) {
      const slot = document.createElement('div');
      slot.className = 'inventory-slot';
      slot.dataset.slot = String(i);

      const item = playerState.inventory[i];
      if (item) {
        slot.innerHTML = `
          <div class="item-icon">${this.getItemIcon(item.item)}</div>
          <div class="item-count">${item.count > 1 ? item.count : ''}</div>
        `;
        slot.title = `${item.item} x${item.count}`;
      }

      // Drag handling
      slot.addEventListener('mousedown', (e) => this.handleSlotClick(i, e));
      slot.addEventListener('mouseup', (e) => this.handleSlotRelease(i, e));

      grid.appendChild(slot);
    }
  }

  private handleSlotClick(slotIndex: number, e: MouseEvent): void {
    e.preventDefault();
    this.dragFromSlot = slotIndex;
  }

  private handleSlotRelease(slotIndex: number, e: MouseEvent): void {
    e.preventDefault();
    if (this.dragFromSlot !== null && this.dragFromSlot !== slotIndex) {
      connection.send({
        type: 'swapSlots',
        data: { from: this.dragFromSlot, to: slotIndex },
      });
    }
    this.dragFromSlot = null;
  }

  private getItemIcon(itemId: string): string {
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

export const inventory = new InventoryManager();
