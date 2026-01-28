import { NetworkManager } from './core/network';
import { GameState } from './core/state';
import { Camera } from './render/camera';
import { Renderer } from './render/renderer';
import { KeyboardManager } from './input/keyboard';
import { TouchManager } from './input/touch';
import { MouseManager } from './input/mouse';
import { InputManager } from './input/manager';
import { HUDManager } from './ui/hud';
import { InventoryManager } from './ui/inventory';
import { MenuManager } from './ui/menu';
import { NotificationManager } from './ui/notifications';
import { LoginManager } from './ui/login';

import type { ServerMessage, PlayerState } from './types';

class GameClient {
  // Core
  private network: NetworkManager | null = null;
  private state: GameState;
  private camera: Camera;

  // Rendering
  private canvas: HTMLCanvasElement;
  private renderer: Renderer;

  // Input
  private keyboard: KeyboardManager;
  private touch: TouchManager;
  private mouse: MouseManager;
  private input: InputManager | null = null;

  // UI
  private hud: HUDManager;
  private inventory: InventoryManager;
  private menu: MenuManager;
  private notifications: NotificationManager;
  private login: LoginManager;

  constructor() {
    // Get canvas
    const canvas = document.getElementById('game-canvas') as HTMLCanvasElement;
    if (!canvas) throw new Error('Canvas not found');
    this.canvas = canvas;

    // Initialize core systems
    this.state = new GameState();
    this.camera = new Camera();

    // Initialize renderer
    this.renderer = new Renderer(this.canvas, this.camera, this.state);

    // Initialize input
    this.keyboard = new KeyboardManager();
    this.touch = new TouchManager(
      document.getElementById('joystick') as HTMLElement,
      document.getElementById('joystick-zone') as HTMLElement,
      document.getElementById('action-zone') as HTMLElement
    );
    this.mouse = new MouseManager(this.canvas, this.camera);

    // Initialize UI (menu must be created before input)
    this.menu = new MenuManager();
    this.hud = new HUDManager();
    this.inventory = new InventoryManager();
    this.notifications = new NotificationManager();
    this.login = new LoginManager({
      onClaimSession: () => this.claimSession(),
      onConnect: () => this.connect(),
    });

    // Setup message handling
    this.setupUIHandlers();
    this.setupKeyboardShortcuts();
    this.startRenderLoop();
  }

  private setupUIHandlers(): void {
    // Hotbar slot selection
    this.hud.onSlotSelect = (slot) => {
      this.network?.send({ type: 'slot', data: { slot } });
    };

    // Inventory slot selection
    this.inventory.onSlotSelect = (slot) => {
      this.network?.send({ type: 'slot', data: { slot } });
    };

    // Menu callbacks
    this.menu.onResume = () => {
      // Just close menu
    };

    this.menu.onInventory = () => {
      const player = this.state.getPlayerState();
      if (player) this.inventory.render(player);
    };

    this.menu.onCrafting = () => {
      // Crafting UI updated separately
    };

    this.menu.onSettings = () => {
      // Settings UI updated separately
    };

    this.menu.onDisconnect = () => {
      this.disconnect();
    };
  }

  private setupKeyboardShortcuts(): void {
    this.keyboard.onKey((key, pressed) => {
      if (!pressed) return;

      switch (key.toLowerCase()) {
        case 'i':
        case 'e':
          if (!this.menu.isOpen()) {
            this.inventory.toggle();
            const player = this.state.getPlayerState();
            if (player && this.inventory.isOpened()) {
              this.inventory.render(player);
            }
          }
          break;

        case 'escape':
          if (this.inventory.isOpened()) {
            this.inventory.close();
          }
          break;

        case '1':
        case '2':
        case '3':
        case '4':
        case '5':
        case '6':
        case '7': {
          const slot = parseInt(key) - 1;
          this.network?.send({ type: 'slot', data: { slot } });
          break;
        }

        case ' ':
          // Space attack handled directly
          if (this.state.isSpawned() && !this.inventory.isOpened() && !this.menu.isOpen()) {
            this.network?.send({ type: 'attack', data: { target_id: 'nearest' } });
          }
          break;
      }
    });
  }

  private async claimSession(): Promise<void> {
    const result = await NetworkManager.claimSession();
    if (result) {
      localStorage.setItem('playerId', result.id);
      localStorage.setItem('sessionToken', result.token);
      this.login.showSessionInfo(result.token);
      this.notifications.show('Session claimed!');
    } else {
      this.notifications.show('Failed to claim session');
    }
  }

  private connect(): void {
    const token = localStorage.getItem('sessionToken');
    if (!token) {
      this.notifications.show('No session token');
      return;
    }

    this.network = new NetworkManager(token);
    this.network.onMessage((msg) => this.handleMessage(msg));
    this.network.connect();

    // Setup input manager after connection
    this.input = new InputManager(
      this.network,
      this.camera,
      this.keyboard,
      this.touch,
      this.mouse,
      this.canvas,
      () => this.menu.isOpen() || this.inventory.isOpened()
    );
  }

  private disconnect(): void {
    this.input?.destroy();
    this.input = null;
    this.network?.disconnect();
    this.network = null;

    this.hud.hide();
    this.login.show();
    this.camera.reset();
  }

  private handleMessage(msg: ServerMessage): void {
    switch (msg.type) {
      case 'welcome':
        if (!msg.spawned) {
          this.network?.send({ type: 'spawn', data: { settlement_id: null } });
        }
        break;

      case 'playerUpdate':
        this.state.setPlayerState(msg.data);
        this.hud.update(msg.data);
        if (this.inventory.isOpened()) {
          this.inventory.render(msg.data);
        }
        if (msg.data.spawned) {
          this.hud.show();
          this.login.hide();
          this.camera.follow(msg.data.position?.x || 0, msg.data.position?.y || 0);
        }
        break;

      case 'chunkAdd':
        this.state.addChunk({
          coord: msg.data.coord,
          biome: msg.data.biome,
          entities: new Map([
            ...Object.entries(msg.data.entities.resources),
            ...Object.entries(msg.data.entities.mobs),
            ...Object.entries(msg.data.entities.structures),
            ...Object.entries(msg.data.entities.npcs),
          ]),
        });
        break;

      case 'chunkRemove':
        this.state.removeChunk(msg.data.coord);
        break;

      case 'entityDelta':
        this.state.updateEntities(
          msg.data.chunk,
          msg.data.updates,
          msg.data.removes
        );
        break;

      case 'notification':
        this.notifications.show(msg.data.text);
        break;

      case 'error':
        this.notifications.show(`Error: ${msg.data.message}`);
        break;

      case 'sessionRevoked':
        this.notifications.show('Session revoked');
        localStorage.removeItem('sessionToken');
        localStorage.removeItem('playerId');
        this.disconnect();
        break;

      case 'combatResult': {
        const crit = msg.data.critical ? ' CRITICAL!' : '';
        this.notifications.show(`Hit: ${msg.data.damage.toFixed(1)} dmg${crit}`);
        break;
      }

      case 'resourceDepleted': {
        const items = msg.data.items_received.map((i) => `${i.count}x ${i.item}`).join(', ');
        this.notifications.show(`Gathered: ${items}`);
        break;
      }

      case 'achievement':
        this.notifications.show(`Achievement: ${msg.data.name}!`);
        break;

      case 'npcInteraction':
        // Show NPC dialog
        this.notifications.show(`${msg.data.name}: ${msg.data.text}`);
        break;

      case 'questUpdate':
        this.notifications.show(`Quest ${msg.data.quest.state}: ${msg.data.quest.name}`);
        break;
    }
  }

  private startRenderLoop(): void {
    const loop = () => {
      // Update camera
      this.camera.update();

      // Render
      this.renderer.render();

      requestAnimationFrame(loop);
    };
    requestAnimationFrame(loop);
  }
}

new GameClient();
