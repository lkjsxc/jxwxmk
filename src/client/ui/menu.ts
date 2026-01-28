export type MenuPage = 'menu' | 'inventory' | 'crafting' | 'settings';

export class MenuManager {
  private menuModal: HTMLElement | null;
  private inventoryModal: HTMLElement | null;
  private craftingModal: HTMLElement | null;
  private settingsModal: HTMLElement | null;

  private currentPage: MenuPage | null = null;

  // Button callbacks
  onResume: (() => void) | null = null;
  onInventory: (() => void) | null = null;
  onCrafting: (() => void) | null = null;
  onSettings: (() => void) | null = null;
  onDisconnect: (() => void) | null = null;

  constructor() {
    this.menuModal = document.getElementById('pause-menu');
    this.inventoryModal = document.getElementById('inventory-modal');
    this.craftingModal = document.getElementById('crafting-modal');
    this.settingsModal = document.getElementById('settings-modal');

    this.setupListeners();
    this.createModals();
    this.setupMenuButton();
  }

  private setupMenuButton(): void {
    const menuBtn = document.getElementById('menu-btn');
    if (menuBtn) {
      menuBtn.addEventListener('click', () => {
        this.toggle();
      });
    }
  }

  private createModals(): void {
    // Create crafting modal if it doesn't exist
    if (!this.craftingModal) {
      this.craftingModal = this.createModal('crafting-modal', 'Crafting', `
        <div id="crafting-grid" class="crafting-grid">
          <div class="crafting-placeholder">Crafting recipes will appear here</div>
        </div>
      `);
      document.getElementById('ui-layer')?.appendChild(this.craftingModal);
    }

    // Create settings modal if it doesn't exist
    if (!this.settingsModal) {
      this.settingsModal = this.createModal('settings-modal', 'Settings', `
        <div class="settings-content">
          <div class="setting-row">
            <label>Sound Effects</label>
            <input type="checkbox" checked disabled />
          </div>
          <div class="setting-row">
            <label>Music</label>
            <input type="checkbox" checked disabled />
          </div>
          <div class="setting-row">
            <label>Graphics Quality</label>
            <select disabled>
              <option>Low</option>
              <option selected>Medium</option>
              <option>High</option>
            </select>
          </div>
        </div>
      `);
      document.getElementById('ui-layer')?.appendChild(this.settingsModal);
    }
  }

  private createModal(id: string, title: string, content: string): HTMLElement {
    const modal = document.createElement('div');
    modal.id = id;
    modal.className = 'modal hidden';
    modal.innerHTML = `
      <div class="modal-content">
        <div class="modal-header">
          <h2>${title}</h2>
          <button class="btn-close" data-close="${id}">&times;</button>
        </div>
        ${content}
      </div>
    `;

    // Add close listener
    modal.querySelector('.btn-close')?.addEventListener('click', () => {
      this.close();
    });

    return modal;
  }

  private setupListeners(): void {
    // Main menu buttons
    document.getElementById('btn-resume')?.addEventListener('click', () => {
      this.close();
      this.onResume?.();
    });

    document.getElementById('btn-inventory')?.addEventListener('click', () => {
      this.showPage('inventory');
      this.onInventory?.();
    });

    document.getElementById('btn-crafting')?.addEventListener('click', () => {
      this.showPage('crafting');
      this.onCrafting?.();
    });

    document.getElementById('btn-settings')?.addEventListener('click', () => {
      this.showPage('settings');
      this.onSettings?.();
    });

    document.getElementById('btn-disconnect')?.addEventListener('click', () => {
      this.close();
      this.onDisconnect?.();
    });

    document.getElementById('close-menu')?.addEventListener('click', () => {
      this.close();
    });

    // Keyboard shortcut
    window.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        if (this.currentPage && this.currentPage !== 'menu') {
          this.showPage('menu');
        } else {
          this.toggle();
        }
      }
    });
  }

  toggle(): void {
    if (this.currentPage) {
      this.close();
    } else {
      this.showPage('menu');
    }
  }

  showPage(page: MenuPage): void {
    // Close all modals first
    this.hideAllModals();

    this.currentPage = page;

    switch (page) {
      case 'menu':
        this.menuModal?.classList.remove('hidden');
        break;
      case 'inventory':
        this.inventoryModal?.classList.remove('hidden');
        break;
      case 'crafting':
        this.craftingModal?.classList.remove('hidden');
        break;
      case 'settings':
        this.settingsModal?.classList.remove('hidden');
        break;
    }
  }

  close(): void {
    this.hideAllModals();
    this.currentPage = null;
  }

  private hideAllModals(): void {
    this.menuModal?.classList.add('hidden');
    this.inventoryModal?.classList.add('hidden');
    this.craftingModal?.classList.add('hidden');
    this.settingsModal?.classList.add('hidden');
  }

  isOpen(): boolean {
    return this.currentPage !== null;
  }

  getCurrentPage(): MenuPage | null {
    return this.currentPage;
  }
}
