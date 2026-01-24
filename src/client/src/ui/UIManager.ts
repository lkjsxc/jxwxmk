import { GameState, CraftingRecipe } from '../game/GameState';

export class UIManager {
    private uiContainer: HTMLElement;
    private loadingScreen: HTMLElement;
    private gameUI: HTMLElement | null;
    private inventoryUI: HTMLElement | null;
    private craftingUI: HTMLElement | null;
    private recipes: CraftingRecipe[] = [];
    
    public onCraftRequest: ((recipeId: string) => void) | null = null;
    
    constructor() {
        this.uiContainer = document.getElementById('ui-container') || document.createElement('div');
        this.loadingScreen = document.getElementById('loading-screen') || document.createElement('div');
        this.gameUI = null;
        this.inventoryUI = null;
        this.craftingUI = null;
    }
    
    public initialize(): void {
        this.loadingScreen.style.display = 'none';
        this.createGameUI();
        this.createInventoryUI();
        this.createCraftingUI();
    }
    
    private createGameUI(): void {
        this.gameUI = document.createElement('div');
        this.gameUI.id = 'game-ui';
        this.gameUI.style.position = 'absolute';
        this.gameUI.style.top = '10px';
        this.gameUI.style.left = '10px';
        this.gameUI.style.color = 'white';
        this.gameUI.style.fontFamily = 'Arial, sans-serif';
        this.gameUI.style.pointerEvents = 'none';
        
        const statsDiv = document.createElement('div');
        statsDiv.id = 'player-stats';
        statsDiv.style.backgroundColor = 'rgba(0, 0, 0, 0.5)';
        statsDiv.style.padding = '10px';
        statsDiv.style.borderRadius = '5px';
        statsDiv.style.marginBottom = '10px';
        statsDiv.innerHTML = "
            <div>Health: <span id=\"health-value\">100</span>/<span id=\"max-health-value\">100</span></div>
            <div>Hunger: <span id=\"hunger-value\">100</span>/<span id=\"max-hunger-value\">100</span></div>
            <div>Thirst: <span id=\"thirst-value\">100</span>/<span id=\"max-thirst-value\">100</span></div>
        ";
        
        this.gameUI.appendChild(statsDiv);
        this.uiContainer.appendChild(this.gameUI);
    }
    
    private createInventoryUI(): void {
        this.inventoryUI = document.createElement('div');
        this.inventoryUI.id = 'inventory-ui';
        this.inventoryUI.style.position = 'absolute';
        this.inventoryUI.style.bottom = '20px';
        this.inventoryUI.style.right = '20px';
        this.inventoryUI.style.backgroundColor = 'rgba(0, 0, 0, 0.7)';
        this.inventoryUI.style.padding = '10px';
        this.inventoryUI.style.borderRadius = '5px';
        this.inventoryUI.style.display = 'none';
        this.inventoryUI.style.pointerEvents = 'auto';
        
        const title = document.createElement('h3');
        title.textContent = 'Inventory';
        title.style.marginTop = '0';
        title.style.color = '#FFD700';
        
        const itemsDiv = document.createElement('div');
        itemsDiv.id = 'inventory-items';
        itemsDiv.style.display = 'grid';
        itemsDiv.style.gridTemplateColumns = 'repeat(5, 50px)';
        itemsDiv.style.gap = '5px';
        
        for (let i = 0; i < 10; i++) {
            const slot = document.createElement('div');
            slot.className = 'inventory-slot';
            slot.style.width = '50px';
            slot.style.height = '50px';
            slot.style.backgroundColor = 'rgba(255, 255, 255, 0.1)';
            slot.style.border = '1px solid #666';
            slot.style.borderRadius = '3px';
            slot.style.display = 'flex';
            slot.style.justifyContent = 'center';
            slot.style.alignItems = 'center';
            slot.style.fontSize = '10px';
            slot.style.textAlign = 'center';
            slot.textContent = 'Empty';
            itemsDiv.appendChild(slot);
        }
        
        this.inventoryUI.appendChild(title);
        this.inventoryUI.appendChild(itemsDiv);
        this.uiContainer.appendChild(this.inventoryUI);
    }
    
    private createCraftingUI(): void {
        this.craftingUI = document.createElement('div');
        this.craftingUI.id = 'crafting-ui';
        this.craftingUI.style.position = 'absolute';
        this.craftingUI.style.top = '50%';
        this.craftingUI.style.left = '50%';
        this.craftingUI.style.transform = 'translate(-50%, -50%)';
        this.craftingUI.style.backgroundColor = 'rgba(0, 0, 0, 0.8)';
        this.craftingUI.style.padding = '20px';
        this.craftingUI.style.borderRadius = '10px';
        this.craftingUI.style.display = 'none';
        this.craftingUI.style.pointerEvents = 'auto';
        this.craftingUI.style.width = '400px';
        
        const title = document.createElement('h2');
        title.textContent = 'Crafting';
        title.style.marginTop = '0';
        title.style.color = '#FFD700';
        title.style.textAlign = 'center';
        
        const recipesDiv = document.createElement('div');
        recipesDiv.id = 'crafting-recipes';
        recipesDiv.style.maxHeight = '300px';
        recipesDiv.style.overflowY = 'auto';
        recipesDiv.style.marginTop = '10px';
        
        const closeButton = document.createElement('button');
        closeButton.textContent = 'Close';
        closeButton.style.display = 'block';
        closeButton.style.margin = '10px auto 0';
        closeButton.style.padding = '8px 16px';
        closeButton.addEventListener('click', () => {
            this.hideCraftingUI();
        });
        
        this.craftingUI.appendChild(title);
        this.craftingUI.appendChild(recipesDiv);
        this.craftingUI.appendChild(closeButton);
        this.uiContainer.appendChild(this.craftingUI);
    }

    public setRecipes(recipes: CraftingRecipe[]): void {
        this.recipes = recipes;
        this.updateCraftingUI();
    }

    private updateCraftingUI(): void {
        const recipesDiv = document.getElementById('crafting-recipes');
        if (!recipesDiv) return;

        recipesDiv.innerHTML = '';
        this.recipes.forEach(recipe => {
            const recipeDiv = document.createElement('div');
            recipeDiv.className = 'crafting-recipe';
            recipeDiv.style.padding = '10px';
            recipeDiv.style.marginBottom = '5px';
            recipeDiv.style.backgroundColor = 'rgba(255, 255, 255, 0.1)';
            recipeDiv.style.borderRadius = '5px';
            recipeDiv.style.cursor = 'pointer';
            
            const reqs = recipe.requirements.map(r => `${r[0]} x${r[1]}`).join(', ');
            
            recipeDiv.innerHTML = "
                <strong>${recipe.name}</strong><br>
                <small>Requires: ${reqs}</small><br>
                <small>Produces: ${recipe.result.item_type} x${recipe.result.quantity}</small>
            ";
            
            recipeDiv.addEventListener('click', () => {
                if (this.onCraftRequest) {
                    this.onCraftRequest(recipe.id);
                }
            });
            
            recipesDiv.appendChild(recipeDiv);
        });
    }
    
    public showGameUI(): void {
        if (this.gameUI) this.gameUI.style.display = 'block';
    }
    
    public hideGameUI(): void {
        if (this.gameUI) this.gameUI.style.display = 'none';
    }
    
    public showInventoryUI(): void {
        if (this.inventoryUI) this.inventoryUI.style.display = 'block';
    }
    
    public hideInventoryUI(): void {
        if (this.inventoryUI) this.inventoryUI.style.display = 'none';
    }
    
    public showCraftingUI(): void {
        if (this.craftingUI) this.craftingUI.style.display = 'block';
    }
    
    public hideCraftingUI(): void {
        if (this.craftingUI) this.craftingUI.style.display = 'none';
    }
    
    public toggleCraftingUI(): void {
        if (this.craftingUI) {
            if (this.craftingUI.style.display === 'none') {
                this.showCraftingUI();
            } else {
                this.hideCraftingUI();
            }
        }
    }

    public showLoading(): void {
        this.loadingScreen.style.display = 'flex';
    }
    
    public hideLoading(): void {
        this.loadingScreen.style.display = 'none';
    }
    
    public showError(message: string): void {
        const errorDiv = document.createElement('div');
        errorDiv.style.position = 'absolute';
        errorDiv.style.top = '50%';
        errorDiv.style.left = '50%';
        errorDiv.style.transform = 'translate(-50%, -50%)';
        errorDiv.style.backgroundColor = 'rgba(255, 0, 0, 0.8)';
        errorDiv.style.color = 'white';
        errorDiv.style.padding = '20px';
        errorDiv.style.borderRadius = '5px';
        errorDiv.style.zIndex = '1001';
        errorDiv.textContent = `Error: ${message}`;
        errorDiv.addEventListener('click', () => errorDiv.remove());
        this.uiContainer.appendChild(errorDiv);
        setTimeout(() => { if (errorDiv.parentNode) errorDiv.remove(); }, 5000);
    }
    
    public showDisconnected(): void {
        const div = document.createElement('div');
        div.id = 'disconnected-overlay';
        div.style.position = 'absolute';
        div.style.top = '0';
        div.style.left = '0';
        div.style.width = '100%';
        div.style.height = '100%';
        div.style.backgroundColor = 'rgba(0, 0, 0, 0.9)';
        div.style.color = 'white';
        div.style.display = 'flex';
        div.style.justifyContent = 'center';
        div.style.alignItems = 'center';
        div.style.zIndex = '1002';
        div.style.fontSize = '24px';
        div.textContent = 'Disconnected from server. Attempting to reconnect...';
        this.uiContainer.appendChild(div);
    }
    
    public hideDisconnected(): void {
        const overlay = document.getElementById('disconnected-overlay');
        if (overlay) overlay.remove();
    }
    
    public render(gameState: GameState): void {
        if (!gameState.player) return;
        
        if (this.gameUI) {
            const h = this.gameUI.querySelector('#health-value');
            const mh = this.gameUI.querySelector('#max-health-value');
            const hu = this.gameUI.querySelector('#hunger-value');
            const mhu = this.gameUI.querySelector('#max-hunger-value');
            const t = this.gameUI.querySelector('#thirst-value');
            const mt = this.gameUI.querySelector('#max-thirst-value');
            
            if (h) h.textContent = Math.round(gameState.player.health).toString();
            if (mh) mh.textContent = Math.round(gameState.player.maxHealth).toString();
            if (hu) hu.textContent = Math.round(gameState.player.hunger).toString();
            if (mhu) mhu.textContent = Math.round(gameState.player.maxHunger).toString();
            if (t) t.textContent = Math.round(gameState.player.thirst).toString();
            if (mt) mt.textContent = Math.round(gameState.player.maxThirst).toString();
        }
        
        if (this.inventoryUI) {
            const slots = this.inventoryUI.querySelectorAll('.inventory-slot');
            const items = Array.from(gameState.player.inventory.entries());
            
            slots.forEach((slot, index) => {
                if (index < items.length) {
                    const [type, qty] = items[index];
                    slot.textContent = `${type}\n(${qty})`;
                } else {
                    slot.textContent = 'Empty';
                }
            });
        }
    }
}
