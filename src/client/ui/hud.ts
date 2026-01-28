import type { PlayerState } from '../types';

export class HUDManager {
  private hpBar: HTMLElement | null;
  private hungerBar: HTMLElement | null;
  private tempBar: HTMLElement | null;
  private hotbarSlots: NodeListOf<HTMLElement>;
  private activeItemName: HTMLElement | null;

  constructor() {
    this.hpBar = document.getElementById('hp-bar');
    this.hungerBar = document.getElementById('hunger-bar');
    this.tempBar = document.getElementById('temp-bar');
    this.hotbarSlots = document.querySelectorAll('.hotbar-slot');
    this.activeItemName = document.getElementById('active-item-name');

    this.setupHotbarClicks();
  }

  private setupHotbarClicks(): void {
    this.hotbarSlots.forEach((slot, index) => {
      slot.addEventListener('click', () => {
        this.onSlotSelect?.(index);
      });
    });
  }

  // Callback for slot selection
  onSlotSelect: ((slot: number) => void) | null = null;

  update(playerState: PlayerState): void {
    // Update vitals bars
    if (this.hpBar) {
      const pct = (playerState.vitals.hp / playerState.vitals.max_hp) * 100;
      this.hpBar.style.width = `${pct}%`;
    }

    if (this.hungerBar) {
      const pct = (playerState.vitals.hunger / playerState.vitals.max_hunger) * 100;
      this.hungerBar.style.width = `${pct}%`;
    }

    if (this.tempBar) {
      const pct = (playerState.vitals.temperature / playerState.vitals.max_temperature) * 100;
      this.tempBar.style.width = `${pct}%`;
    }

    // Update hotbar
    this.updateHotbar(playerState.active_slot, playerState.inventory);
  }

  private updateHotbar(
    activeSlot: number,
    inventory: Array<{ item: string; count: number } | null>
  ): void {
    // Update slot highlighting
    this.hotbarSlots.forEach((slot, index) => {
      slot.classList.toggle('active', index === activeSlot);

      // Update slot content
      const item = inventory[index];
      if (item) {
        slot.textContent = item.count > 1 ? `${item.count}` : '';
        slot.setAttribute('title', item.item);
      } else {
        slot.textContent = (index + 1).toString();
        slot.removeAttribute('title');
      }
    });

    // Update active item name
    const activeItem = inventory[activeSlot];
    if (this.activeItemName) {
      this.activeItemName.textContent = activeItem ? activeItem.item : '';
    }
  }

  show(): void {
    document.getElementById('hud')?.classList.remove('hidden');
  }

  hide(): void {
    document.getElementById('hud')?.classList.add('hidden');
  }
}
