import { connection } from '../connection';
import { input } from '../input';
import { RECIPES } from '../types';
import type { PlayerState } from '../types';

export class CraftingManager {
  private modal: HTMLElement | null = null;
  private isOpen = false;
  private selectedRecipe: string | null = null;

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
    this.modal.id = 'crafting-modal';
    this.modal.className = 'modal';
    this.modal.innerHTML = `
      <div class="modal-content crafting-content">
        <div class="modal-header">
          <h2>Crafting</h2>
          <button class="close-btn">&times;</button>
        </div>
        <div class="crafting-body">
          <div class="recipe-list" id="recipe-list"></div>
          <div class="recipe-details" id="recipe-details"></div>
        </div>
      </div>
    `;

    document.body.appendChild(this.modal);
    this.isOpen = true;
    input.registerModal('crafting');

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
    this.selectedRecipe = null;
    input.unregisterModal('crafting');
  }

  update(state: PlayerState): void {
    if (this.isOpen) {
      this.render(state);
    }
  }

  private render(state?: PlayerState): void {
    if (!this.modal) return;

    const playerState = state || (window as unknown as { playerState?: PlayerState }).playerState;

    const list = this.modal.querySelector('#recipe-list') as HTMLElement;
    const details = this.modal.querySelector('#recipe-details') as HTMLElement;

    // Render recipe list
    list.innerHTML = '';
    for (const recipe of RECIPES) {
      const canCraft = playerState ? this.canCraft(recipe, playerState) : false;

      const recipeEl = document.createElement('div');
      recipeEl.className = `recipe-item ${this.selectedRecipe === recipe.id ? 'selected' : ''} ${canCraft ? '' : 'disabled'}`;
      recipeEl.textContent = recipe.name;
      recipeEl.addEventListener('click', () => {
        this.selectedRecipe = recipe.id;
        this.render(playerState);
      });
      list.appendChild(recipeEl);
    }

    // Render recipe details
    if (this.selectedRecipe) {
      const recipe = RECIPES.find(r => r.id === this.selectedRecipe);
      if (recipe) {
        const canCraft = playerState ? this.canCraft(recipe, playerState) : false;

        details.innerHTML = `
          <h3>${recipe.name}</h3>
          <div class="requirements">
            <h4>Requirements:</h4>
            ${recipe.requirements.map(req => {
              const have = playerState ? this.countItem(req.item, playerState) : 0;
              const met = have >= req.count;
              return `<div class="requirement ${met ? 'met' : ''}">${this.getItemIcon(req.item)} ${req.item}: ${have}/${req.count}</div>`;
            }).join('')}
          </div>
          <button class="craft-btn" ${canCraft ? '' : 'disabled'}>Craft</button>
        `;

        details.querySelector('.craft-btn')!.addEventListener('click', () => {
          if (canCraft) {
            connection.send({ type: 'craft', data: { recipe: recipe.id } });
          }
        });
      }
    } else {
      details.innerHTML = '<p>Select a recipe to view details</p>';
    }
  }

  private canCraft(recipe: typeof RECIPES[0], state: PlayerState): boolean {
    for (const req of recipe.requirements) {
      if (this.countItem(req.item, state) < req.count) {
        return false;
      }
    }
    return true;
  }

  private countItem(itemId: string, state: PlayerState): number {
    let count = 0;
    for (const slot of state.inventory) {
      if (slot && slot.item === itemId) {
        count += slot.count;
      }
    }
    return count;
  }

  private getItemIcon(itemId: string): string {
    const icons: Record<string, string> = {
      wood: '🪵',
      stone: '🪨',
    };
    return icons[itemId] || '📦';
  }
}

export const crafting = new CraftingManager();
