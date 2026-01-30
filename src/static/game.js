"use strict";
(() => {
  var __defProp = Object.defineProperty;
  var __getOwnPropNames = Object.getOwnPropertyNames;
  var __esm = (fn, res) => function __init() {
    return fn && (res = (0, fn[__getOwnPropNames(fn)[0]])(fn = 0)), res;
  };
  var __export = (target, all) => {
    for (var name in all)
      __defProp(target, name, { get: all[name], enumerable: true });
  };

  // ui/notifications.ts
  var notifications_exports = {};
  __export(notifications_exports, {
    NotificationManager: () => NotificationManager,
    notifications: () => notifications
  });
  var NOTIFICATION_DURATION_MS, NotificationManager, notifications;
  var init_notifications = __esm({
    "ui/notifications.ts"() {
      "use strict";
      NOTIFICATION_DURATION_MS = 3e3;
      NotificationManager = class {
        constructor() {
          this.notifications = [];
          this.nextId = 1;
          this.container = document.getElementById("notifications");
        }
        show(text, type = "info") {
          const notification = {
            id: this.nextId++,
            text,
            type,
            createdAt: Date.now()
          };
          this.notifications.push(notification);
          this.render();
          setTimeout(() => {
            this.remove(notification.id);
          }, NOTIFICATION_DURATION_MS);
        }
        remove(id) {
          const idx = this.notifications.findIndex((n) => n.id === id);
          if (idx >= 0) {
            this.notifications.splice(idx, 1);
            this.render();
          }
        }
        render() {
          this.container.innerHTML = "";
          const notification = this.notifications[this.notifications.length - 1];
          if (!notification)
            return;
          const el = document.createElement("div");
          el.className = `notification notification-${notification.type}`;
          el.textContent = notification.text;
          if (notification.type === "error") {
            el.style.borderLeftColor = "#ff4444";
          } else if (notification.type === "achievement") {
            el.style.borderLeftColor = "#ffd700";
          }
          this.container.appendChild(el);
        }
        clear() {
          this.notifications = [];
          this.render();
        }
      };
      notifications = new NotificationManager();
    }
  });

  // connection.ts
  var KEEPALIVE_INTERVAL_MS = 5e3;
  var ConnectionManager = class {
    constructor() {
      this.ws = null;
      this.token = null;
      this.messageHandlers = [];
      this.closeHandlers = [];
      this.reconnectAttempts = 0;
      this.maxReconnectAttempts = 3;
      this.reconnectDelay = 1e3;
      this.keepaliveInterval = null;
      this.lastActivity = Date.now();
    }
    get isConnected() {
      return this.ws?.readyState === WebSocket.OPEN;
    }
    get sessionToken() {
      return this.token;
    }
    setSessionToken(token) {
      this.token = token;
      if (token) {
        localStorage.setItem("jxwxmk_token", token);
      } else {
        localStorage.removeItem("jxwxmk_token");
      }
    }
    loadStoredToken() {
      const stored = localStorage.getItem("jxwxmk_token");
      if (stored) {
        this.token = stored;
      }
      return this.token;
    }
    clearToken() {
      this.token = null;
      localStorage.removeItem("jxwxmk_token");
    }
    onMessage(handler) {
      this.messageHandlers.push(handler);
      return () => {
        const idx = this.messageHandlers.indexOf(handler);
        if (idx >= 0)
          this.messageHandlers.splice(idx, 1);
      };
    }
    onClose(handler) {
      this.closeHandlers.push(handler);
      return () => {
        const idx = this.closeHandlers.indexOf(handler);
        if (idx >= 0)
          this.closeHandlers.splice(idx, 1);
      };
    }
    connect() {
      if (this.ws?.readyState === WebSocket.CONNECTING)
        return;
      if (this.ws?.readyState === WebSocket.OPEN)
        return;
      const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
      const host = window.location.host;
      const tokenParam = this.token ? `?token=${encodeURIComponent(this.token)}` : "";
      const url = `${protocol}//${host}/ws${tokenParam}`;
      console.log("[Connection] Connecting to:", url);
      this.ws = new WebSocket(url);
      this.ws.onopen = () => {
        console.log("[Connection] WebSocket connected");
        this.reconnectAttempts = 0;
        this.lastActivity = Date.now();
        this.startKeepalive();
      };
      this.ws.onmessage = (event) => {
        this.lastActivity = Date.now();
        try {
          const msg = JSON.parse(event.data);
          this.handleMessage(msg);
        } catch (err) {
          console.error("[Connection] Failed to parse message:", err);
        }
      };
      this.ws.onclose = (event) => {
        console.log("[Connection] WebSocket closed:", event.code, event.reason);
        this.stopKeepalive();
        this.ws = null;
        this.closeHandlers.forEach((h) => h(event.code, event.reason));
      };
      this.ws.onerror = (err) => {
        console.error("[Connection] WebSocket error:", err);
      };
    }
    disconnect() {
      this.stopKeepalive();
      if (this.ws) {
        this.ws.close();
        this.ws = null;
      }
    }
    send(msg) {
      if (!this.isConnected)
        return false;
      try {
        this.ws.send(JSON.stringify(msg));
        this.lastActivity = Date.now();
        return true;
      } catch (err) {
        console.error("[Connection] Failed to send message:", err);
        return false;
      }
    }
    startKeepalive() {
      this.stopKeepalive();
      this.keepaliveInterval = window.setInterval(() => {
        if (!this.isConnected) {
          this.stopKeepalive();
          return;
        }
        const timeSinceActivity = Date.now() - this.lastActivity;
        if (timeSinceActivity > KEEPALIVE_INTERVAL_MS - 1e3) {
          this.send({
            type: "input",
            data: { dx: 0, dy: 0, attack: false, interact: false }
          });
        }
      }, KEEPALIVE_INTERVAL_MS);
    }
    stopKeepalive() {
      if (this.keepaliveInterval !== null) {
        clearInterval(this.keepaliveInterval);
        this.keepaliveInterval = null;
      }
    }
    handleMessage(msg) {
      if (msg.type === "welcome") {
        this.setSessionToken(msg.token);
      }
      if (msg.type === "sessionRevoked") {
        this.clearToken();
      }
      this.messageHandlers.forEach((h) => h(msg));
    }
  };
  var connection = new ConnectionManager();

  // input.ts
  var INPUT_INTERVAL_MS = 50;
  var ATTACK_COOLDOWN_MS = 500;
  var INTERACT_COOLDOWN_MS = 400;
  var LONG_PRESS_MS = 275;
  var InputManager = class {
    constructor() {
      // Movement
      this.dx = 0;
      this.dy = 0;
      // Actions
      this.attack = false;
      this.interact = false;
      // Pointer state
      this.mouseX = 0;
      this.mouseY = 0;
      this.pointers = /* @__PURE__ */ new Map();
      this.isPointerDown = false;
      this.pressStartTime = 0;
      // Cooldowns
      this.lastAttackTime = 0;
      this.lastInteractTime = 0;
      // Joystick (touch)
      this.joystickActive = false;
      this.joystickCenterX = 0;
      this.joystickCenterY = 0;
      this.joystickCurrentX = 0;
      this.joystickCurrentY = 0;
      this.joystickMaxRadius = 50;
      // Input loop
      this.intervalId = null;
      // World-space aim point
      this.aimX = 0;
      this.aimY = 0;
      // Camera reference for screen-to-world conversion
      this.cameraX = 0;
      this.cameraY = 0;
      this.zoom = 1;
      // UI State tracking
      this.modalsOpen = /* @__PURE__ */ new Set();
      // ===== Keyboard Handling =====
      this.keys = /* @__PURE__ */ new Set();
      // Zoom callback
      this.emitZoom = null;
      this.setupKeyboard();
      this.setupMouse();
      this.setupTouch();
      this.setupVisibilityHandling();
    }
    setCamera(x, y, zoom) {
      this.cameraX = x;
      this.cameraY = y;
      this.zoom = zoom;
    }
    registerModal(name) {
      this.modalsOpen.add(name);
    }
    unregisterModal(name) {
      this.modalsOpen.delete(name);
    }
    start() {
      if (this.intervalId !== null)
        return;
      this.intervalId = window.setInterval(() => this.tick(), INPUT_INTERVAL_MS);
    }
    stop() {
      if (this.intervalId !== null) {
        clearInterval(this.intervalId);
        this.intervalId = null;
      }
    }
    tick() {
      if (this.joystickActive) {
        const dx = this.joystickCurrentX - this.joystickCenterX;
        const dy = this.joystickCurrentY - this.joystickCenterY;
        const dist = Math.sqrt(dx * dx + dy * dy);
        if (dist > 5) {
          const normalized = Math.min(dist, this.joystickMaxRadius) / this.joystickMaxRadius;
          this.dx = dx / dist * normalized;
          this.dy = dy / dist * normalized;
        } else {
          this.dx = 0;
          this.dy = 0;
        }
      }
      if (this.isPointerDown && !this.interact && !this.attack) {
        const pressDuration = Date.now() - this.pressStartTime;
        if (pressDuration >= LONG_PRESS_MS && this.canInteract()) {
          this.interact = true;
          this.updateAim();
        }
      }
      const msg = {
        type: "input",
        data: {
          dx: this.dx,
          dy: this.dy,
          attack: this.attack,
          interact: this.interact
        }
      };
      if (this.attack || this.interact) {
        msg.data.aim = { x: this.aimX, y: this.aimY };
      }
      connection.send(msg);
      this.attack = false;
      this.interact = false;
    }
    canAttack() {
      return Date.now() - this.lastAttackTime >= ATTACK_COOLDOWN_MS;
    }
    canInteract() {
      return Date.now() - this.lastInteractTime >= INTERACT_COOLDOWN_MS;
    }
    updateAim() {
      const canvas = document.getElementById("game-canvas");
      if (!canvas)
        return;
      const rect = canvas.getBoundingClientRect();
      const centerX = rect.width / 2;
      const centerY = rect.height / 2;
      const screenDx = this.mouseX - centerX;
      const screenDy = this.mouseY - centerY;
      const PPU3 = 16;
      this.aimX = this.cameraX + screenDx / (PPU3 * this.zoom);
      this.aimY = this.cameraY + screenDy / (PPU3 * this.zoom);
    }
    setupKeyboard() {
      window.addEventListener("keydown", (e) => {
        if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
          return;
        }
        this.keys.add(e.key.toLowerCase());
        this.updateMovementFromKeys();
        if (e.key >= "1" && e.key <= "7") {
          const slot = parseInt(e.key) - 1;
          connection.send({ type: "slot", data: { slot } });
        }
        if (e.key.toLowerCase() === "e" && this.canInteract()) {
          this.interact = true;
          this.updateAim();
          this.lastInteractTime = Date.now();
        }
      });
      window.addEventListener("keyup", (e) => {
        this.keys.delete(e.key.toLowerCase());
        this.updateMovementFromKeys();
      });
    }
    updateMovementFromKeys() {
      if (this.joystickActive)
        return;
      let dx = 0;
      let dy = 0;
      if (this.keys.has("w") || this.keys.has("arrowup"))
        dy -= 1;
      if (this.keys.has("s") || this.keys.has("arrowdown"))
        dy += 1;
      if (this.keys.has("a") || this.keys.has("arrowleft"))
        dx -= 1;
      if (this.keys.has("d") || this.keys.has("arrowright"))
        dx += 1;
      if (dx !== 0 && dy !== 0) {
        const len = Math.sqrt(dx * dx + dy * dy);
        dx /= len;
        dy /= len;
      }
      this.dx = dx;
      this.dy = dy;
    }
    // ===== Mouse Handling =====
    setupMouse() {
      window.addEventListener("mousemove", (e) => {
        this.mouseX = e.clientX;
        this.mouseY = e.clientY;
      });
      window.addEventListener("mousedown", (e) => {
        if (e.button !== 0)
          return;
        if (this.isClickOnUI(e.clientX, e.clientY))
          return;
        this.isPointerDown = true;
        this.pressStartTime = Date.now();
        this.mouseX = e.clientX;
        this.mouseY = e.clientY;
        if (this.canAttack()) {
          this.attack = true;
          this.updateAim();
          this.lastAttackTime = Date.now();
        }
      });
      window.addEventListener("mouseup", () => {
        this.isPointerDown = false;
      });
      window.addEventListener("wheel", (e) => {
        const delta = e.deltaY > 0 ? -0.1 : 0.1;
        this.emitZoom?.(delta);
      }, { passive: true });
      window.addEventListener("contextmenu", (e) => {
        e.preventDefault();
      });
    }
    isClickOnUI(x, y) {
      if (this.modalsOpen.size > 0)
        return true;
      const target = document.elementFromPoint(x, y);
      if (!target)
        return false;
      if (target.closest(".modal"))
        return true;
      if (target.closest(".overlay"))
        return true;
      if (target.closest("#login-screen"))
        return true;
      return false;
    }
    // ===== Touch Handling =====
    setupTouch() {
      const canvas = document.getElementById("game-canvas");
      if (!canvas)
        return;
      canvas.addEventListener("touchstart", (e) => {
        e.preventDefault();
        for (let i = 0; i < e.changedTouches.length; i++) {
          const touch = e.changedTouches[i];
          const x = touch.clientX;
          const y = touch.clientY;
          const screenWidth = window.innerWidth;
          if (x < screenWidth / 2 && !this.joystickActive) {
            this.joystickActive = true;
            this.joystickCenterX = x;
            this.joystickCenterY = y;
            this.joystickCurrentX = x;
            this.joystickCurrentY = y;
          }
          if (x >= screenWidth / 2) {
            this.isPointerDown = true;
            this.pressStartTime = Date.now();
            this.mouseX = x;
            this.mouseY = y;
            if (this.canAttack()) {
              this.attack = true;
              this.updateAim();
              this.lastAttackTime = Date.now();
            }
          }
          this.pointers.set(touch.identifier, {
            id: touch.identifier,
            x,
            y,
            startX: x,
            startY: y,
            startTime: Date.now(),
            isDown: true,
            isLongPress: false
          });
        }
      }, { passive: false });
      canvas.addEventListener("touchmove", (e) => {
        e.preventDefault();
        for (let i = 0; i < e.changedTouches.length; i++) {
          const touch = e.changedTouches[i];
          const pointer = this.pointers.get(touch.identifier);
          if (!pointer)
            continue;
          pointer.x = touch.clientX;
          pointer.y = touch.clientY;
          if (this.joystickActive && pointer.startX < window.innerWidth / 2) {
            const dx = touch.clientX - this.joystickCenterX;
            const dy = touch.clientY - this.joystickCenterY;
            const dist = Math.sqrt(dx * dx + dy * dy);
            if (dist > this.joystickMaxRadius) {
              const angle = Math.atan2(dy, dx);
              this.joystickCurrentX = this.joystickCenterX + Math.cos(angle) * this.joystickMaxRadius;
              this.joystickCurrentY = this.joystickCenterY + Math.sin(angle) * this.joystickMaxRadius;
            } else {
              this.joystickCurrentX = touch.clientX;
              this.joystickCurrentY = touch.clientY;
            }
          }
          if (pointer.x >= window.innerWidth / 2) {
            this.mouseX = touch.clientX;
            this.mouseY = touch.clientY;
          }
        }
      }, { passive: false });
      canvas.addEventListener("touchend", (e) => {
        e.preventDefault();
        for (let i = 0; i < e.changedTouches.length; i++) {
          const touch = e.changedTouches[i];
          const pointer = this.pointers.get(touch.identifier);
          if (!pointer)
            continue;
          if (this.joystickActive && pointer.startX < window.innerWidth / 2) {
            this.joystickActive = false;
            this.dx = 0;
            this.dy = 0;
          }
          if (pointer.startX >= window.innerWidth / 2) {
            this.isPointerDown = false;
          }
          this.pointers.delete(touch.identifier);
        }
      });
      canvas.addEventListener("touchcancel", (e) => {
        this.pointers.clear();
        this.joystickActive = false;
        this.isPointerDown = false;
        this.dx = 0;
        this.dy = 0;
      });
    }
    // ===== Visibility Handling =====
    setupVisibilityHandling() {
      document.addEventListener("visibilitychange", () => {
        if (document.visibilityState === "visible") {
          if (connection.isConnected && this.intervalId === null) {
            this.start();
          }
        }
      });
      window.addEventListener("blur", () => {
        this.keys.clear();
        this.dx = 0;
        this.dy = 0;
      });
    }
  };
  var input = new InputManager();

  // camera.ts
  var PPU = 16;
  var MIN_ZOOM = 0.75;
  var MAX_ZOOM = 2;
  var DEFAULT_ZOOM = 1.1;
  var FOLLOW_FACTOR = 0.1;
  var Camera = class {
    constructor() {
      this.x = 0;
      this.y = 0;
      this.zoom = DEFAULT_ZOOM;
      this.targetX = 0;
      this.targetY = 0;
      this.hasInitialTarget = false;
    }
    follow(targetX, targetY) {
      this.targetX = targetX;
      this.targetY = targetY;
      if (!this.hasInitialTarget) {
        this.x = targetX;
        this.y = targetY;
        this.hasInitialTarget = true;
      }
    }
    update() {
      this.x += (this.targetX - this.x) * FOLLOW_FACTOR;
      this.y += (this.targetY - this.y) * FOLLOW_FACTOR;
    }
    adjustZoom(delta) {
      this.zoom = Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, this.zoom + delta));
    }
    // Convert world coordinates to screen coordinates
    worldToScreen(worldX, worldY, canvasWidth, canvasHeight) {
      const screenX = (worldX - this.x) * PPU * this.zoom + canvasWidth / 2;
      const screenY = (worldY - this.y) * PPU * this.zoom + canvasHeight / 2;
      return { x: screenX, y: screenY };
    }
    // Convert screen coordinates to world coordinates
    screenToWorld(screenX, screenY, canvasWidth, canvasHeight) {
      const worldX = (screenX - canvasWidth / 2) / (PPU * this.zoom) + this.x;
      const worldY = (screenY - canvasHeight / 2) / (PPU * this.zoom) + this.y;
      return { x: worldX, y: worldY };
    }
    // Get viewport bounds in world coordinates
    getViewport(canvasWidth, canvasHeight) {
      const halfViewWidth = canvasWidth / 2 / (PPU * this.zoom);
      const halfViewHeight = canvasHeight / 2 / (PPU * this.zoom);
      return {
        minX: this.x - halfViewWidth,
        minY: this.y - halfViewHeight,
        maxX: this.x + halfViewWidth,
        maxY: this.y + halfViewHeight
      };
    }
    reset() {
      this.x = 0;
      this.y = 0;
      this.zoom = DEFAULT_ZOOM;
      this.hasInitialTarget = false;
    }
  };
  var camera = new Camera();

  // world.ts
  var CHUNK_SIZE = 128;
  var WorldManager = class {
    constructor() {
      this.chunks = /* @__PURE__ */ new Map();
      // key: "x,y"
      this.playerEntity = null;
      this.playerMissingSince = null;
      this.playerTimeoutMs = 3e3;
    }
    // 3 seconds before game over
    // Get chunk key from coordinates
    getChunkKey(coord) {
      return `${coord[0]},${coord[1]}`;
    }
    // Get chunk coordinates from world position
    getChunkCoord(worldX, worldY) {
      const cx = Math.floor(worldX / CHUNK_SIZE);
      const cy = Math.floor(worldY / CHUNK_SIZE);
      return [cx, cy];
    }
    // Add or update a chunk
    addChunk(chunk) {
      const key = this.getChunkKey(chunk.coord);
      this.chunks.set(key, chunk);
    }
    // Remove a chunk
    removeChunk(coord) {
      const key = this.getChunkKey(coord);
      this.chunks.delete(key);
    }
    // Apply entity delta to a chunk
    applyEntityDelta(delta) {
      const key = this.getChunkKey(delta.chunk);
      const chunk = this.chunks.get(key);
      if (!chunk)
        return;
      for (const update of delta.updates) {
        const existing = this.findEntity(update.id);
        if (existing) {
          update.prevX = existing.x;
          update.prevY = existing.y;
          update.lastUpdate = Date.now();
        }
        this.addEntityToChunk(chunk, update);
      }
      for (const removal of delta.removes) {
        this.removeEntityFromChunk(chunk, removal.id, removal.kind);
      }
    }
    findEntity(id) {
      for (const chunk of this.chunks.values()) {
        for (const collection of [chunk.entities.resources, chunk.entities.mobs, chunk.entities.structures, chunk.entities.npcs]) {
          const found = collection.find((e) => e.id === id);
          if (found)
            return found;
        }
      }
      return null;
    }
    addEntityToChunk(chunk, entity) {
      const collection = this.getEntityCollection(chunk, entity.kind);
      if (collection) {
        const idx = collection.findIndex((e) => e.id === entity.id);
        if (idx >= 0) {
          collection[idx] = entity;
        } else {
          collection.push(entity);
        }
      }
      if (entity.kind === "player" && entity.id === this.playerEntity?.id) {
        this.playerEntity = entity;
        this.playerMissingSince = null;
      }
    }
    removeEntityFromChunk(chunk, id, kind) {
      const collection = this.getEntityCollection(chunk, kind);
      if (collection) {
        const idx = collection.findIndex((e) => e.id === id);
        if (idx >= 0) {
          collection.splice(idx, 1);
        }
      }
      if (kind === "player" && id === this.playerEntity?.id) {
        this.playerMissingSince = Date.now();
      }
    }
    getEntityCollection(chunk, kind) {
      switch (kind) {
        case "resource":
          return chunk.entities.resources;
        case "mob":
          return chunk.entities.mobs;
        case "structure":
          return chunk.entities.structures;
        case "npc":
          return chunk.entities.npcs;
        case "player":
          return [];
        default:
          return null;
      }
    }
    // Get all visible entities (for rendering)
    getVisibleEntities() {
      const entities = [];
      for (const chunk of this.chunks.values()) {
        for (const collection of [chunk.entities.resources, chunk.entities.mobs, chunk.entities.structures, chunk.entities.npcs]) {
          entities.push(...collection);
        }
      }
      if (this.playerEntity) {
        entities.push(this.playerEntity);
      }
      return entities;
    }
    // Get entities near a point (for targeting)
    getEntitiesNear(x, y, radius) {
      const results = [];
      const entities = this.getVisibleEntities();
      for (const entity of entities) {
        const dx = entity.x - x;
        const dy = entity.y - y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        if (dist <= radius) {
          results.push(entity);
        }
      }
      return results.sort((a, b) => {
        const da = Math.sqrt((a.x - x) ** 2 + (a.y - y) ** 2);
        const db = Math.sqrt((b.x - x) ** 2 + (b.y - y) ** 2);
        return da - db;
      });
    }
    // Set player entity ID for tracking
    setPlayerId(id) {
      this.playerEntity = { id, kind: "player", subtype: "player", x: 0, y: 0 };
    }
    // Get player entity
    getPlayerEntity() {
      return this.playerEntity;
    }
    // Check if player should be in game over state
    isGameOver() {
      return this.playerMissingSince !== null && Date.now() - this.playerMissingSince > this.playerTimeoutMs;
    }
    // Reset game over state
    resetGameOver() {
      this.playerMissingSince = null;
    }
    // Clear all chunks (on disconnect)
    clear() {
      this.chunks.clear();
      this.playerEntity = null;
      this.playerMissingSince = null;
    }
    // Get interpolated entity position
    getInterpolatedPosition(entity, now) {
      if (entity.prevX === void 0 || entity.lastUpdate === void 0) {
        return { x: entity.x, y: entity.y };
      }
      const elapsed = now - entity.lastUpdate;
      const duration = 100;
      const t = Math.min(1, elapsed / duration);
      return {
        x: entity.prevX + (entity.x - entity.prevX) * t,
        y: entity.prevY + (entity.y - entity.prevY) * t
      };
    }
  };
  var world = new WorldManager();

  // renderer.ts
  var PPU2 = 16;
  var ENTITY_COLORS = {
    player: "#6a6aff",
    tree: "#2d5a27",
    rock: "#666666",
    berry_bush: "#8b4513",
    wolf: "#8b0000",
    bear: "#5c4033",
    wall: "#8b7355",
    door: "#654321",
    workbench: "#a0522d",
    torch: "#ff8c00",
    npc: "#ffd700"
  };
  var Renderer = class {
    constructor() {
      this.animationId = null;
      this.lastHitEntities = /* @__PURE__ */ new Map();
      this.loop = () => {
        this.animationId = requestAnimationFrame(this.loop);
        this.render();
      };
      this.canvas = document.getElementById("game-canvas");
      this.ctx = this.canvas.getContext("2d");
      this.resize();
      window.addEventListener("resize", () => this.resize());
    }
    resize() {
      this.canvas.width = window.innerWidth;
      this.canvas.height = window.innerHeight;
    }
    start() {
      if (this.animationId !== null)
        return;
      this.loop();
    }
    stop() {
      if (this.animationId !== null) {
        cancelAnimationFrame(this.animationId);
        this.animationId = null;
      }
    }
    render() {
      const ctx = this.ctx;
      const width = this.canvas.width;
      const height = this.canvas.height;
      ctx.fillStyle = "#0f0f1a";
      ctx.fillRect(0, 0, width, height);
      camera.update();
      input.setCamera(camera.x, camera.y, camera.zoom);
      this.drawGrid(width, height);
      const entities = world.getVisibleEntities();
      const now = Date.now();
      entities.sort((a, b) => a.y - b.y);
      for (const entity of entities) {
        this.drawEntity(entity, now);
      }
      this.drawTargetingHighlight(width, height);
      this.drawJoystick();
    }
    drawGrid(width, viewportHeight) {
      const ctx = this.ctx;
      const viewport = camera.getViewport(width, viewportHeight);
      ctx.strokeStyle = "#1a1a2e";
      ctx.lineWidth = 1;
      const gridSize = 10;
      const startX = Math.floor(viewport.minX / gridSize) * gridSize;
      const endX = Math.ceil(viewport.maxX / gridSize) * gridSize;
      const startY = Math.floor(viewport.minY / gridSize) * gridSize;
      const endY = Math.ceil(viewport.maxY / gridSize) * gridSize;
      ctx.beginPath();
      for (let x = startX; x <= endX; x += gridSize) {
        const screenX = camera.worldToScreen(x, 0, width, viewportHeight).x;
        ctx.moveTo(screenX, 0);
        ctx.lineTo(screenX, viewportHeight);
      }
      for (let y = startY; y <= endY; y += gridSize) {
        const screenY = camera.worldToScreen(0, y, width, viewportHeight).y;
        ctx.moveTo(0, screenY);
        ctx.lineTo(width, screenY);
      }
      ctx.stroke();
    }
    drawEntity(entity, now) {
      const ctx = this.ctx;
      const width = this.canvas.width;
      const height = this.canvas.height;
      const pos = world.getInterpolatedPosition(entity, now);
      const screenPos = camera.worldToScreen(pos.x, pos.y, width, height);
      let radiusWu = 0.75;
      if (entity.kind === "resource")
        radiusWu = 1;
      if (entity.kind === "mob")
        radiusWu = 0.9;
      if (entity.kind === "npc")
        radiusWu = 0.8;
      if (entity.kind === "structure")
        radiusWu = 1.25;
      const radiusPx = radiusWu * PPU2 * camera.zoom;
      let color = ENTITY_COLORS[entity.subtype] || ENTITY_COLORS[entity.kind] || "#888";
      const lastHit = this.lastHitEntities.get(entity.id);
      if (lastHit && now - lastHit < 250) {
        const t = (now - lastHit) / 250;
        const scale = 1 + Math.sin(t * Math.PI) * 0.15;
        ctx.save();
        ctx.translate(screenPos.x, screenPos.y);
        ctx.scale(scale, scale);
        ctx.translate(-screenPos.x, -screenPos.y);
        this.lastHitEntities.set(entity.id, lastHit);
      }
      ctx.fillStyle = color;
      ctx.beginPath();
      ctx.arc(screenPos.x, screenPos.y, radiusPx, 0, Math.PI * 2);
      ctx.fill();
      ctx.strokeStyle = "#000";
      ctx.lineWidth = 2;
      ctx.stroke();
      if (entity.hp !== void 0 && entity.max_hp !== void 0 && entity.hp < entity.max_hp) {
        const barWidth = radiusPx * 2;
        const barHeight = 4;
        const hpPct = entity.hp / entity.max_hp;
        ctx.fillStyle = "#333";
        ctx.fillRect(screenPos.x - barWidth / 2, screenPos.y - radiusPx - 10, barWidth, barHeight);
        ctx.fillStyle = hpPct > 0.5 ? "#0f0" : hpPct > 0.25 ? "#ff0" : "#f00";
        ctx.fillRect(screenPos.x - barWidth / 2, screenPos.y - radiusPx - 10, barWidth * hpPct, barHeight);
      }
      if (entity.kind === "mob" && entity.level !== void 0) {
        ctx.fillStyle = "#fff";
        ctx.font = `bold ${10 * camera.zoom}px sans-serif`;
        ctx.textAlign = "center";
        ctx.fillText(`Lv.${entity.level}`, screenPos.x, screenPos.y - radiusPx - 15);
      }
      if ((entity.kind === "npc" || entity.kind === "player") && entity.name) {
        ctx.fillStyle = "#fff";
        ctx.font = `${12 * camera.zoom}px sans-serif`;
        ctx.textAlign = "center";
        ctx.fillText(entity.name, screenPos.x, screenPos.y - radiusPx - 15);
      }
      if (lastHit && now - lastHit < 250) {
        ctx.restore();
      }
    }
    drawTargetingHighlight(width, height) {
      const ctx = this.ctx;
      const aim = camera.screenToWorld(input.mouseX, input.mouseY, width, height);
      const nearby = world.getEntitiesNear(aim.x, aim.y, 4);
      const target = nearby[0];
      if (target) {
        const pos = world.getInterpolatedPosition(target, Date.now());
        const screenPos = camera.worldToScreen(pos.x, pos.y, width, height);
        let radiusWu = 0.75;
        if (target.kind === "resource")
          radiusWu = 1;
        if (target.kind === "mob")
          radiusWu = 0.9;
        if (target.kind === "npc")
          radiusWu = 0.8;
        if (target.kind === "structure")
          radiusWu = 1.25;
        const radiusPx = radiusWu * PPU2 * camera.zoom;
        ctx.strokeStyle = "#6a6aff";
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.arc(screenPos.x, screenPos.y, radiusPx + 4, 0, Math.PI * 2);
        ctx.stroke();
        const tooltipText = this.getInteractionText(target);
        ctx.fillStyle = "rgba(0, 0, 0, 0.8)";
        ctx.font = "12px sans-serif";
        const textWidth = ctx.measureText(tooltipText).width;
        ctx.fillRect(screenPos.x - textWidth / 2 - 4, screenPos.y - radiusPx - 35, textWidth + 8, 18);
        ctx.fillStyle = "#fff";
        ctx.textAlign = "center";
        ctx.fillText(tooltipText, screenPos.x, screenPos.y - radiusPx - 22);
      }
    }
    getInteractionText(entity) {
      if (entity.kind === "resource")
        return `Gather ${entity.subtype}`;
      if (entity.kind === "mob")
        return "Attack";
      if (entity.kind === "npc")
        return "Talk";
      if (entity.kind === "structure")
        return "Interact";
      return "Interact";
    }
    drawJoystick() {
      if (!input["joystickActive"])
        return;
      const ctx = this.ctx;
      const centerX = input["joystickCenterX"];
      const centerY = input["joystickCenterY"];
      const currentX = input["joystickCurrentX"];
      const currentY = input["joystickCurrentY"];
      ctx.strokeStyle = "rgba(255, 255, 255, 0.3)";
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.arc(centerX, centerY, 50, 0, Math.PI * 2);
      ctx.stroke();
      ctx.fillStyle = "rgba(255, 255, 255, 0.5)";
      ctx.beginPath();
      ctx.arc(currentX, currentY, 20, 0, Math.PI * 2);
      ctx.fill();
    }
    triggerHitFlash(entityId) {
      this.lastHitEntities.set(entityId, Date.now());
    }
  };
  var renderer = new Renderer();

  // ui/screens.ts
  var ScreenManager = class {
    constructor() {
      this.currentScreen = "login";
      this.loginScreen = document.getElementById("login-screen");
      this.hud = document.getElementById("hud");
      this.setupLoginScreen();
    }
    setupLoginScreen() {
      const connectBtn = document.getElementById("connect-btn");
      const playerIdInput = document.getElementById("player-id");
      connectBtn.addEventListener("click", () => {
        const playerId = playerIdInput.value.trim();
        this.doConnect(playerId || null);
      });
      playerIdInput.addEventListener("keypress", (e) => {
        if (e.key === "Enter") {
          const playerId = playerIdInput.value.trim();
          this.doConnect(playerId || null);
        }
      });
    }
    async doConnect(playerId) {
      if (playerId) {
        try {
          const response = await fetch("/session/claim", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ player_id: playerId })
          });
          if (!response.ok) {
            const error = await response.json();
            alert(`Failed to claim session: ${error.message || "Unknown error"}`);
            return;
          }
          const data = await response.json();
          connection.setSessionToken(data.token);
        } catch (err) {
          console.error("Failed to claim session:", err);
          alert("Failed to connect to server");
          return;
        }
      } else {
        connection.clearToken();
      }
      connection.connect();
    }
    show(screen) {
      this.currentScreen = screen;
      this.loginScreen.classList.add("hidden");
      this.hud.classList.add("hidden");
      switch (screen) {
        case "login":
          this.loginScreen.classList.remove("hidden");
          break;
        case "game":
          this.hud.classList.remove("hidden");
          break;
        case "gameover":
          this.showGameOver();
          break;
        case "session_revoked":
          this.showSessionRevoked();
          break;
      }
    }
    showGameOver() {
      let overlay = document.getElementById("gameover-overlay");
      if (!overlay) {
        overlay = document.createElement("div");
        overlay.id = "gameover-overlay";
        overlay.className = "overlay";
        overlay.innerHTML = `
        <div class="overlay-content">
          <h1>YOU DIED</h1>
          <button id="respawn-btn">Respawn</button>
        </div>
      `;
        document.body.appendChild(overlay);
        const respawnBtn = document.getElementById("respawn-btn");
        respawnBtn.addEventListener("click", () => {
          connection.send({ type: "spawn", data: { settlement_id: null } });
          overlay.remove();
        });
      }
    }
    showSessionRevoked() {
      connection.clearToken();
      let overlay = document.getElementById("revoked-overlay");
      if (!overlay) {
        overlay = document.createElement("div");
        overlay.id = "revoked-overlay";
        overlay.className = "overlay";
        overlay.innerHTML = `
        <div class="overlay-content">
          <h1>Session Revoked</h1>
          <p>Your session was logged in elsewhere.</p>
          <button id="reconnect-btn">Reconnect</button>
        </div>
      `;
        document.body.appendChild(overlay);
        const reconnectBtn = document.getElementById("reconnect-btn");
        reconnectBtn.addEventListener("click", () => {
          overlay.remove();
          this.show("login");
        });
      }
    }
    getCurrentScreen() {
      return this.currentScreen;
    }
  };
  var screens = new ScreenManager();

  // ui/hud.ts
  var HUDManager = class {
    constructor() {
      this.hpBar = document.getElementById("hp-bar");
      this.hungerBar = document.getElementById("hunger-bar");
      this.tempBar = document.getElementById("temp-bar");
    }
    update(state2) {
      const { vitals } = state2;
      const hpPct = vitals.hp / vitals.max_hp * 100;
      this.hpBar.style.width = `${Math.max(0, Math.min(100, hpPct))}%`;
      const hungerPct = vitals.hunger / vitals.max_hunger * 100;
      this.hungerBar.style.width = `${Math.max(0, Math.min(100, hungerPct))}%`;
      const tempPct = vitals.temperature / vitals.max_temperature * 100;
      this.tempBar.style.width = `${Math.max(0, Math.min(100, tempPct))}%`;
      if (vitals.hp < vitals.max_hp * 0.25) {
        this.hpBar.style.background = "#ff0000";
      } else {
        this.hpBar.style.background = "#ff4444";
      }
      if (vitals.hunger < vitals.max_hunger * 0.25) {
        this.hungerBar.style.background = "#ff6600";
      } else {
        this.hungerBar.style.background = "#ffaa44";
      }
      if (vitals.temperature < 20 || vitals.temperature > 80) {
        this.tempBar.style.background = "#ff4444";
      } else {
        this.tempBar.style.background = "#4444ff";
      }
    }
    show() {
      const hud2 = document.getElementById("hud");
      hud2.classList.remove("hidden");
    }
    hide() {
      const hud2 = document.getElementById("hud");
      hud2.classList.add("hidden");
    }
  };
  var hud = new HUDManager();

  // ui/hotbar.ts
  var HotbarManager = class {
    constructor() {
      this.slots = [];
      this.activeSlot = 0;
      this.container = document.getElementById("hotbar");
      this.createSlots();
    }
    createSlots() {
      this.container.innerHTML = "";
      this.slots = [];
      for (let i = 0; i < 7; i++) {
        const slot = document.createElement("div");
        slot.className = "hotbar-slot";
        slot.dataset.slot = String(i);
        slot.title = `Slot ${i + 1} (Press ${i + 1})`;
        slot.addEventListener("click", () => {
          this.selectSlot(i);
        });
        this.container.appendChild(slot);
        this.slots.push(slot);
      }
    }
    update(state2) {
      this.activeSlot = state2.active_slot;
      for (let i = 0; i < 7; i++) {
        const slotEl = this.slots[i];
        const item = state2.inventory[i];
        if (i === this.activeSlot) {
          slotEl.classList.add("active");
        } else {
          slotEl.classList.remove("active");
        }
        if (item) {
          slotEl.innerHTML = `
          <div class="item-icon">${this.getItemIcon(item.item)}</div>
          <div class="item-count">${item.count > 1 ? item.count : ""}</div>
        `;
          slotEl.title = `${item.item} x${item.count}`;
        } else {
          slotEl.innerHTML = "";
          slotEl.title = `Slot ${i + 1}`;
        }
      }
    }
    selectSlot(slot) {
      if (slot < 0 || slot >= 7)
        return;
      connection.send({ type: "slot", data: { slot } });
    }
    getItemIcon(itemId) {
      const icons = {
        wood: "\u{1FAB5}",
        stone: "\u{1FAA8}",
        wood_pickaxe: "\u26CF\uFE0F",
        stone_pickaxe: "\u2692\uFE0F",
        wood_wall: "\u{1F9F1}",
        door: "\u{1F6AA}",
        torch: "\u{1F525}",
        workbench: "\u{1F528}",
        berry: "\u{1F352}",
        meat: "\u{1F356}"
      };
      return icons[itemId] || "\u{1F4E6}";
    }
  };
  var hotbar = new HotbarManager();

  // ui/inventory.ts
  var INVENTORY_SIZE = 30;
  var InventoryManager = class {
    constructor() {
      this.modal = null;
      this.isOpen = false;
      this.dragFromSlot = null;
    }
    toggle() {
      if (this.isOpen) {
        this.close();
      } else {
        this.open();
      }
    }
    open() {
      if (this.modal)
        return;
      this.modal = document.createElement("div");
      this.modal.id = "inventory-modal";
      this.modal.className = "modal";
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
      input.registerModal("inventory");
      this.modal.querySelector(".close-btn").addEventListener("click", () => this.close());
      this.modal.addEventListener("click", (e) => {
        if (e.target === this.modal)
          this.close();
      });
      const escHandler = (e) => {
        if (e.key === "Escape") {
          this.close();
          window.removeEventListener("keydown", escHandler);
        }
      };
      window.addEventListener("keydown", escHandler);
      this.render();
    }
    close() {
      if (this.modal) {
        this.modal.remove();
        this.modal = null;
      }
      this.isOpen = false;
      this.dragFromSlot = null;
      input.unregisterModal("inventory");
    }
    update(state2) {
      if (this.isOpen) {
        this.render(state2);
      }
    }
    render(state2) {
      if (!this.modal)
        return;
      const playerState = state2 || window.playerState;
      if (!playerState)
        return;
      const grid = this.modal.querySelector("#inventory-grid");
      grid.innerHTML = "";
      const cols = window.innerWidth > 600 ? 7 : window.innerWidth > 400 ? 5 : 3;
      grid.style.gridTemplateColumns = `repeat(${cols}, 1fr)`;
      for (let i = 0; i < INVENTORY_SIZE; i++) {
        const slot = document.createElement("div");
        slot.className = "inventory-slot";
        slot.dataset.slot = String(i);
        const item = playerState.inventory[i];
        if (item) {
          slot.innerHTML = `
          <div class="item-icon">${this.getItemIcon(item.item)}</div>
          <div class="item-count">${item.count > 1 ? item.count : ""}</div>
        `;
          slot.title = `${item.item} x${item.count}`;
        }
        slot.addEventListener("mousedown", (e) => this.handleSlotClick(i, e));
        slot.addEventListener("mouseup", (e) => this.handleSlotRelease(i, e));
        grid.appendChild(slot);
      }
    }
    handleSlotClick(slotIndex, e) {
      e.preventDefault();
      this.dragFromSlot = slotIndex;
    }
    handleSlotRelease(slotIndex, e) {
      e.preventDefault();
      if (this.dragFromSlot !== null && this.dragFromSlot !== slotIndex) {
        connection.send({
          type: "swapSlots",
          data: { from: this.dragFromSlot, to: slotIndex }
        });
      }
      this.dragFromSlot = null;
    }
    getItemIcon(itemId) {
      const icons = {
        wood: "\u{1FAB5}",
        stone: "\u{1FAA8}",
        wood_pickaxe: "\u26CF\uFE0F",
        stone_pickaxe: "\u2692\uFE0F",
        wood_wall: "\u{1F9F1}",
        door: "\u{1F6AA}",
        torch: "\u{1F525}",
        workbench: "\u{1F528}",
        berry: "\u{1F352}",
        meat: "\u{1F356}"
      };
      return icons[itemId] || "\u{1F4E6}";
    }
  };
  var inventory = new InventoryManager();

  // types.ts
  var RECIPES = [
    { id: "WoodPickaxe", name: "Wood Pick", requirements: [{ item: "wood", count: 10 }] },
    { id: "StonePickaxe", name: "Stone Pick", requirements: [{ item: "wood", count: 10 }, { item: "stone", count: 10 }] },
    { id: "WoodWall", name: "Wood Wall", requirements: [{ item: "wood", count: 20 }] },
    { id: "Door", name: "Door", requirements: [{ item: "wood", count: 30 }] },
    { id: "Torch", name: "Torch", requirements: [{ item: "wood", count: 2 }] },
    { id: "Workbench", name: "Workbench", requirements: [{ item: "wood", count: 50 }] }
  ];
  var ALL_ACHIEVEMENTS = [
    { id: "first_steps", name: "First Steps" },
    { id: "gatherer", name: "Gatherer" },
    { id: "craftsman", name: "Craftsman" },
    { id: "survivor", name: "Survivor" },
    { id: "warrior", name: "Warrior" }
  ];

  // ui/crafting.ts
  var CraftingManager = class {
    constructor() {
      this.modal = null;
      this.isOpen = false;
      this.selectedRecipe = null;
    }
    toggle() {
      if (this.isOpen) {
        this.close();
      } else {
        this.open();
      }
    }
    open() {
      if (this.modal)
        return;
      this.modal = document.createElement("div");
      this.modal.id = "crafting-modal";
      this.modal.className = "modal";
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
      input.registerModal("crafting");
      this.modal.querySelector(".close-btn").addEventListener("click", () => this.close());
      this.modal.addEventListener("click", (e) => {
        if (e.target === this.modal)
          this.close();
      });
      const escHandler = (e) => {
        if (e.key === "Escape") {
          this.close();
          window.removeEventListener("keydown", escHandler);
        }
      };
      window.addEventListener("keydown", escHandler);
      this.render();
    }
    close() {
      if (this.modal) {
        this.modal.remove();
        this.modal = null;
      }
      this.isOpen = false;
      this.selectedRecipe = null;
      input.unregisterModal("crafting");
    }
    update(state2) {
      if (this.isOpen) {
        this.render(state2);
      }
    }
    render(state2) {
      if (!this.modal)
        return;
      const playerState = state2 || window.playerState;
      const list = this.modal.querySelector("#recipe-list");
      const details = this.modal.querySelector("#recipe-details");
      list.innerHTML = "";
      for (const recipe of RECIPES) {
        const canCraft = playerState ? this.canCraft(recipe, playerState) : false;
        const recipeEl = document.createElement("div");
        recipeEl.className = `recipe-item ${this.selectedRecipe === recipe.id ? "selected" : ""} ${canCraft ? "" : "disabled"}`;
        recipeEl.textContent = recipe.name;
        recipeEl.addEventListener("click", () => {
          this.selectedRecipe = recipe.id;
          this.render(playerState);
        });
        list.appendChild(recipeEl);
      }
      if (this.selectedRecipe) {
        const recipe = RECIPES.find((r) => r.id === this.selectedRecipe);
        if (recipe) {
          const canCraft = playerState ? this.canCraft(recipe, playerState) : false;
          details.innerHTML = `
          <h3>${recipe.name}</h3>
          <div class="requirements">
            <h4>Requirements:</h4>
            ${recipe.requirements.map((req) => {
            const have = playerState ? this.countItem(req.item, playerState) : 0;
            const met = have >= req.count;
            return `<div class="requirement ${met ? "met" : ""}">${this.getItemIcon(req.item)} ${req.item}: ${have}/${req.count}</div>`;
          }).join("")}
          </div>
          <button class="craft-btn" ${canCraft ? "" : "disabled"}>Craft</button>
        `;
          details.querySelector(".craft-btn").addEventListener("click", () => {
            if (canCraft) {
              connection.send({ type: "craft", data: { recipe: recipe.id } });
            }
          });
        }
      } else {
        details.innerHTML = "<p>Select a recipe to view details</p>";
      }
    }
    canCraft(recipe, state2) {
      for (const req of recipe.requirements) {
        if (this.countItem(req.item, state2) < req.count) {
          return false;
        }
      }
      return true;
    }
    countItem(itemId, state2) {
      let count = 0;
      for (const slot of state2.inventory) {
        if (slot && slot.item === itemId) {
          count += slot.count;
        }
      }
      return count;
    }
    getItemIcon(itemId) {
      const icons = {
        wood: "\u{1FAB5}",
        stone: "\u{1FAA8}"
      };
      return icons[itemId] || "\u{1F4E6}";
    }
  };
  var crafting = new CraftingManager();

  // ui/quests.ts
  var QuestManager = class {
    constructor() {
      this.modal = null;
      this.isOpen = false;
    }
    toggle() {
      if (this.isOpen) {
        this.close();
      } else {
        this.open();
      }
    }
    open() {
      if (this.modal)
        return;
      this.modal = document.createElement("div");
      this.modal.id = "quests-modal";
      this.modal.className = "modal";
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
      input.registerModal("quests");
      this.modal.querySelector(".close-btn").addEventListener("click", () => this.close());
      this.modal.addEventListener("click", (e) => {
        if (e.target === this.modal)
          this.close();
      });
      const escHandler = (e) => {
        if (e.key === "Escape") {
          this.close();
          window.removeEventListener("keydown", escHandler);
        }
      };
      window.addEventListener("keydown", escHandler);
      this.render();
    }
    close() {
      if (this.modal) {
        this.modal.remove();
        this.modal = null;
      }
      this.isOpen = false;
      input.unregisterModal("quests");
    }
    update(state2) {
      if (this.isOpen) {
        this.render(state2);
      }
    }
    render(state2) {
      if (!this.modal)
        return;
      const playerState = state2 || window.playerState;
      const list = this.modal.querySelector("#quests-list");
      if (!playerState || playerState.quests.length === 0) {
        list.innerHTML = '<p class="empty">No active quests</p>';
        return;
      }
      list.innerHTML = playerState.quests.map((quest) => this.renderQuest(quest)).join("");
    }
    renderQuest(quest) {
      const stateClass = quest.state.toLowerCase();
      const objectives = quest.objectives?.map((obj) => {
        const pct = obj.current / obj.target * 100;
        return `
        <div class="objective">
          <span>${obj.description}</span>
          <div class="progress-bar">
            <div class="progress-fill" style="width: ${pct}%"></div>
          </div>
          <span>${obj.current}/${obj.target}</span>
        </div>
      `;
      }).join("") || "";
      return `
      <div class="quest-card ${stateClass}">
        <div class="quest-header">
          <h3>${quest.name}</h3>
          <span class="quest-state">${this.formatState(quest.state)}</span>
        </div>
        ${quest.description ? `<p class="quest-description">${quest.description}</p>` : ""}
        ${objectives ? `<div class="quest-objectives">${objectives}</div>` : ""}
      </div>
    `;
    }
    formatState(state2) {
      switch (state2) {
        case "NotStarted":
          return "Not Started";
        case "InProgress":
          return "In Progress";
        case "ReadyToTurnIn":
          return "Ready to Turn In";
        case "Completed":
          return "Completed";
        default:
          return state2;
      }
    }
  };
  var quests = new QuestManager();

  // ui/achievements.ts
  var AchievementsManager = class {
    constructor() {
      this.modal = null;
      this.isOpen = false;
    }
    toggle() {
      if (this.isOpen) {
        this.close();
      } else {
        this.open();
      }
    }
    open() {
      if (this.modal)
        return;
      this.modal = document.createElement("div");
      this.modal.id = "achievements-modal";
      this.modal.className = "modal";
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
      input.registerModal("achievements");
      this.modal.querySelector(".close-btn").addEventListener("click", () => this.close());
      this.modal.addEventListener("click", (e) => {
        if (e.target === this.modal)
          this.close();
      });
      const escHandler = (e) => {
        if (e.key === "Escape") {
          this.close();
          window.removeEventListener("keydown", escHandler);
        }
      };
      window.addEventListener("keydown", escHandler);
      this.render();
    }
    close() {
      if (this.modal) {
        this.modal.remove();
        this.modal = null;
      }
      this.isOpen = false;
      input.unregisterModal("achievements");
    }
    update(state2) {
      if (this.isOpen) {
        this.render(state2);
      }
    }
    render(state2) {
      if (!this.modal)
        return;
      const playerState = state2 || window.playerState;
      const list = this.modal.querySelector("#achievements-list");
      const unlocked = new Set(playerState?.achievements || []);
      list.innerHTML = ALL_ACHIEVEMENTS.map((ach) => {
        const isUnlocked = unlocked.has(ach.id);
        return `
        <div class="achievement-item ${isUnlocked ? "unlocked" : "locked"}">
          <div class="achievement-icon">${isUnlocked ? "\u{1F3C6}" : "\u{1F512}"}</div>
          <div class="achievement-info">
            <h4>${ach.name}</h4>
            <p>${isUnlocked ? "Unlocked!" : "Locked"}</p>
          </div>
        </div>
      `;
      }).join("");
    }
    showToast(name) {
      Promise.resolve().then(() => (init_notifications(), notifications_exports)).then(({ notifications: notifications2 }) => {
        notifications2.show(`Achievement unlocked: ${name}`, "achievement");
      });
    }
  };
  var achievements = new AchievementsManager();

  // ui/profile.ts
  var ProfileManager = class {
    constructor() {
      this.modal = null;
      this.isOpen = false;
    }
    toggle() {
      if (this.isOpen) {
        this.close();
      } else {
        this.open();
      }
    }
    open() {
      if (this.modal)
        return;
      this.modal = document.createElement("div");
      this.modal.id = "profile-modal";
      this.modal.className = "modal";
      this.modal.innerHTML = `
      <div class="modal-content">
        <div class="modal-header">
          <h2>Profile</h2>
          <button class="close-btn">&times;</button>
        </div>
        <div class="profile-body" id="profile-body">
          <div class="profile-section">
            <h3>Player ID</h3>
            <div class="player-id-row">
              <code id="player-id-display">-</code>
              <button id="copy-id-btn">Copy</button>
            </div>
          </div>
          <div class="profile-section">
            <h3>Progression</h3>
            <div class="progression-info">
              <div>Level: <span id="profile-level">-</span></div>
              <div>XP: <span id="profile-xp">-</span></div>
            </div>
          </div>
          <div class="profile-section">
            <h3>Stats</h3>
            <div class="stats-grid" id="stats-grid"></div>
          </div>
          <div class="profile-section">
            <h3>Name</h3>
            <div class="name-row">
              <input type="text" id="name-input" maxlength="20" placeholder="Enter name">
              <button id="update-name-btn">Update</button>
            </div>
          </div>
          <div class="profile-section">
            <h3>Device Login</h3>
            <div class="device-login-row">
              <input type="text" id="device-login-input" placeholder="Enter Player ID">
              <button id="device-login-btn">Login</button>
            </div>
          </div>
        </div>
      </div>
    `;
      document.body.appendChild(this.modal);
      this.isOpen = true;
      input.registerModal("profile");
      this.modal.querySelector(".close-btn").addEventListener("click", () => this.close());
      this.modal.addEventListener("click", (e) => {
        if (e.target === this.modal)
          this.close();
      });
      const escHandler = (e) => {
        if (e.key === "Escape") {
          this.close();
          window.removeEventListener("keydown", escHandler);
        }
      };
      window.addEventListener("keydown", escHandler);
      this.modal.querySelector("#copy-id-btn").addEventListener("click", () => this.copyPlayerId());
      this.modal.querySelector("#update-name-btn").addEventListener("click", () => this.updateName());
      this.modal.querySelector("#device-login-btn").addEventListener("click", () => this.deviceLogin());
      this.render();
    }
    close() {
      if (this.modal) {
        this.modal.remove();
        this.modal = null;
      }
      this.isOpen = false;
      input.unregisterModal("profile");
    }
    update(state2) {
      if (this.isOpen) {
        this.render(state2);
      }
    }
    render(state2) {
      if (!this.modal)
        return;
      const playerState = state2 || window.playerState;
      if (!playerState)
        return;
      const idDisplay = this.modal.querySelector("#player-id-display");
      idDisplay.textContent = playerState.id;
      const levelEl = this.modal.querySelector("#profile-level");
      const xpEl = this.modal.querySelector("#profile-xp");
      levelEl.textContent = String(playerState.level);
      xpEl.textContent = String(playerState.xp);
      const statsGrid = this.modal.querySelector("#stats-grid");
      const stats = [
        { label: "Steps", value: playerState.stats.steps },
        { label: "Kills", value: playerState.stats.kills },
        { label: "Crafts", value: playerState.stats.crafts },
        { label: "Gathers", value: playerState.stats.gathers },
        { label: "Deaths", value: playerState.stats.deaths }
      ];
      statsGrid.innerHTML = stats.map((s) => `
      <div class="stat-item">
        <span class="stat-label">${s.label}</span>
        <span class="stat-value">${s.value}</span>
      </div>
    `).join("");
      const nameInput = this.modal.querySelector("#name-input");
      nameInput.value = playerState.name;
    }
    copyPlayerId() {
      const playerState = window.playerState;
      if (playerState?.id) {
        navigator.clipboard.writeText(playerState.id).then(() => {
          Promise.resolve().then(() => (init_notifications(), notifications_exports)).then(({ notifications: notifications2 }) => {
            notifications2.show("Player ID copied to clipboard");
          });
        });
      }
    }
    updateName() {
      const nameInput = this.modal?.querySelector("#name-input");
      if (!nameInput)
        return;
      const name = nameInput.value.trim();
      if (name) {
        connection.send({ type: "name", data: { name } });
      }
    }
    async deviceLogin() {
      const input2 = this.modal?.querySelector("#device-login-input");
      if (!input2)
        return;
      const playerId = input2.value.trim();
      if (!playerId)
        return;
      try {
        const response = await fetch("/session/claim", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ player_id: playerId })
        });
        if (!response.ok) {
          const error = await response.json();
          Promise.resolve().then(() => (init_notifications(), notifications_exports)).then(({ notifications: notifications2 }) => {
            notifications2.show(`Login failed: ${error.message || "Unknown error"}`, "error");
          });
          return;
        }
        const data = await response.json();
        connection.setSessionToken(data.token);
        connection.disconnect();
        connection.connect();
        this.close();
        Promise.resolve().then(() => (init_notifications(), notifications_exports)).then(({ notifications: notifications2 }) => {
          notifications2.show("Logged in successfully");
        });
      } catch (err) {
        Promise.resolve().then(() => (init_notifications(), notifications_exports)).then(({ notifications: notifications2 }) => {
          notifications2.show("Failed to connect to server", "error");
        });
      }
    }
  };
  var profile = new ProfileManager();

  // ui/manager.ts
  init_notifications();
  var UIManager = class {
    constructor() {
      this.playerState = null;
      this.currentScreen = "login";
      this.setupKeyboardShortcuts();
      this.addStyles();
    }
    setupKeyboardShortcuts() {
      window.addEventListener("keydown", (e) => {
        if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
          return;
        }
        if (e.key.toLowerCase() === "i") {
          inventory.toggle();
        }
        if (e.key.toLowerCase() === "c") {
          crafting.toggle();
        }
        if (e.key.toLowerCase() === "q") {
          quests.toggle();
        }
        if (e.key.toLowerCase() === "p") {
          profile.toggle();
        }
        if (e.key === "Escape") {
          inventory.close();
          crafting.close();
          quests.close();
          achievements.close();
          profile.close();
        }
      });
      const menuBtn = document.getElementById("menu-btn");
      if (menuBtn) {
        menuBtn.addEventListener("click", () => {
          profile.toggle();
        });
      }
      const invBtn = document.getElementById("inv-btn");
      if (invBtn) {
        invBtn.addEventListener("click", () => inventory.toggle());
      }
      const craftBtn = document.getElementById("craft-btn");
      if (craftBtn) {
        craftBtn.addEventListener("click", () => crafting.toggle());
      }
      const questBtn = document.getElementById("quest-btn");
      if (questBtn) {
        questBtn.addEventListener("click", () => quests.toggle());
      }
    }
    addStyles() {
      const style = document.createElement("style");
      style.textContent = `
      /* Modal base styles */
      .modal {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.7);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
      }

      .modal-content {
        background: #1a1a2e;
        border: 2px solid #4a4a6a;
        border-radius: 10px;
        min-width: 300px;
        max-width: 90vw;
        max-height: 80vh;
        overflow: auto;
      }

      .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 15px 20px;
        border-bottom: 1px solid #4a4a6a;
      }

      .modal-header h2 {
        margin: 0;
        font-size: 20px;
      }

      .close-btn {
        background: none;
        border: none;
        color: #fff;
        font-size: 24px;
        cursor: pointer;
      }

      /* Inventory */
      .inventory-grid {
        display: grid;
        gap: 5px;
        padding: 15px;
        max-width: 500px;
      }

      .inventory-slot {
        aspect-ratio: 1;
        background: #2a2a3e;
        border: 2px solid #4a4a6a;
        border-radius: 5px;
        display: flex;
        align-items: center;
        justify-content: center;
        position: relative;
        cursor: pointer;
        font-size: 20px;
      }

      .inventory-slot:hover {
        border-color: #6a6aff;
      }

      .item-count {
        position: absolute;
        bottom: 2px;
        right: 4px;
        font-size: 10px;
        color: #fff;
      }

      /* Crafting */
      .crafting-content {
        min-width: 500px;
      }

      .crafting-body {
        display: flex;
        min-height: 300px;
      }

      .recipe-list {
        width: 40%;
        border-right: 1px solid #4a4a6a;
        overflow-y: auto;
      }

      .recipe-item {
        padding: 10px 15px;
        cursor: pointer;
        border-bottom: 1px solid #2a2a3e;
      }

      .recipe-item:hover {
        background: #2a2a3e;
      }

      .recipe-item.selected {
        background: #3a3a5e;
      }

      .recipe-item.disabled {
        opacity: 0.5;
      }

      .recipe-details {
        flex: 1;
        padding: 20px;
      }

      .requirements {
        margin: 15px 0;
      }

      .requirement {
        margin: 5px 0;
        color: #ff6666;
      }

      .requirement.met {
        color: #66ff66;
      }

      .craft-btn {
        padding: 10px 20px;
        background: #6a6aff;
        color: #fff;
        border: none;
        border-radius: 5px;
        cursor: pointer;
      }

      .craft-btn:disabled {
        background: #4a4a6a;
        cursor: not-allowed;
      }

      /* Quests */
      .quests-list {
        padding: 15px;
      }

      .empty {
        text-align: center;
        color: #888;
        padding: 30px;
      }

      .quest-card {
        background: #2a2a3e;
        border: 1px solid #4a4a6a;
        border-radius: 5px;
        padding: 15px;
        margin-bottom: 10px;
      }

      .quest-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 10px;
      }

      .quest-header h3 {
        margin: 0;
        font-size: 16px;
      }

      .quest-state {
        font-size: 12px;
        padding: 3px 8px;
        background: #4a4a6a;
        border-radius: 3px;
      }

      .quest-card.completed .quest-state {
        background: #2a5a2a;
      }

      .quest-objectives {
        margin-top: 10px;
      }

      .objective {
        display: flex;
        align-items: center;
        gap: 10px;
        margin: 5px 0;
        font-size: 12px;
      }

      .progress-bar {
        flex: 1;
        height: 6px;
        background: #333;
        border-radius: 3px;
        overflow: hidden;
      }

      .progress-fill {
        height: 100%;
        background: #6a6aff;
      }

      /* Achievements */
      .achievements-list {
        padding: 15px;
      }

      .achievement-item {
        display: flex;
        align-items: center;
        gap: 15px;
        padding: 15px;
        border-bottom: 1px solid #2a2a3e;
      }

      .achievement-icon {
        font-size: 24px;
      }

      .achievement-info h4 {
        margin: 0 0 5px 0;
      }

      .achievement-info p {
        margin: 0;
        font-size: 12px;
        color: #888;
      }

      .achievement-item.unlocked .achievement-info p {
        color: #6a6aff;
      }

      /* Profile */
      .profile-body {
        padding: 20px;
      }

      .profile-section {
        margin-bottom: 20px;
      }

      .profile-section h3 {
        margin: 0 0 10px 0;
        font-size: 14px;
        color: #888;
      }

      .player-id-row {
        display: flex;
        gap: 10px;
        align-items: center;
      }

      .player-id-row code {
        background: #2a2a3e;
        padding: 8px 12px;
        border-radius: 5px;
        font-family: monospace;
        font-size: 12px;
        flex: 1;
        overflow: hidden;
        text-overflow: ellipsis;
      }

      .stats-grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 10px;
      }

      .stat-item {
        background: #2a2a3e;
        padding: 10px;
        border-radius: 5px;
        text-align: center;
      }

      .stat-label {
        display: block;
        font-size: 11px;
        color: #888;
        margin-bottom: 5px;
      }

      .stat-value {
        font-size: 18px;
        font-weight: bold;
      }

      .name-row, .device-login-row {
        display: flex;
        gap: 10px;
      }

      .name-row input, .device-login-row input {
        flex: 1;
        padding: 8px 12px;
        background: #2a2a3e;
        border: 1px solid #4a4a6a;
        border-radius: 5px;
        color: #fff;
      }

      .name-row button, .device-login-row button, .player-id-row button {
        padding: 8px 16px;
        background: #6a6aff;
        color: #fff;
        border: none;
        border-radius: 5px;
        cursor: pointer;
      }

      /* Overlays */
      .overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.9);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 2000;
      }

      .overlay-content {
        text-align: center;
      }

      .overlay-content h1 {
        font-size: 48px;
        color: #ff4444;
        margin-bottom: 20px;
      }

      .overlay-content button {
        padding: 15px 30px;
        font-size: 18px;
        background: #6a6aff;
        color: #fff;
        border: none;
        border-radius: 5px;
        cursor: pointer;
      }
    `;
      document.head.appendChild(style);
    }
    // Update all UI components with new player state
    updatePlayerState(state2) {
      this.playerState = state2;
      window.playerState = state2;
      hud.update(state2);
      hotbar.update(state2);
      inventory.update(state2);
      crafting.update(state2);
      quests.update(state2);
      achievements.update(state2);
      profile.update(state2);
    }
    // Show notification
    showNotification(msg) {
      notifications.show(msg.data.text, "info");
    }
    // Show achievement
    showAchievement(msg) {
      achievements.showToast(msg.data.name);
    }
    // Show error
    showError(msg) {
      notifications.show(msg.data.message, "error");
    }
    // Change game screen
    showScreen(screen) {
      this.currentScreen = screen;
      screens.show(screen);
      if (screen === "game") {
        hud.show();
      } else {
        hud.hide();
      }
    }
    getPlayerState() {
      return this.playerState;
    }
    getCurrentScreen() {
      return this.currentScreen;
    }
  };
  var ui = new UIManager();

  // index.ts
  var state = {
    playerId: null,
    playerName: "",
    spawned: false,
    connected: false
  };
  function handleWelcome(msg) {
    state.playerId = msg.id;
    state.spawned = msg.spawned;
    state.connected = true;
    console.log("[Client] Welcome received, player ID:", msg.id);
    if (!msg.spawned) {
      connection.send({ type: "spawn", data: { settlement_id: null } });
    }
    ui.showScreen("game");
    input.start();
    renderer.start();
  }
  function handlePlayerUpdate(msg) {
    const data = msg.data;
    state.playerId = data.id;
    state.playerName = data.name;
    state.spawned = data.spawned;
    if (data.spawned) {
      world.setPlayerId(data.id);
      camera.follow(data.x, data.y);
    }
    ui.updatePlayerState(data);
  }
  function handleChunkAdd(msg) {
    world.addChunk(msg.data);
  }
  function handleChunkRemove(msg) {
    world.removeChunk(msg.data.coord);
  }
  function handleEntityDelta(msg) {
    world.applyEntityDelta(msg.data);
    if (world.isGameOver()) {
      ui.showScreen("gameover");
    }
  }
  function handleSessionRevoked(msg) {
    console.log("[Client] Session revoked:", msg.reason);
    state.connected = false;
    state.spawned = false;
    input.stop();
    renderer.stop();
    world.clear();
    ui.showScreen("session_revoked");
  }
  function handleAchievement(msg) {
    ui.showAchievement(msg);
  }
  function handleNotification(msg) {
    ui.showNotification(msg);
  }
  function handleError(msg) {
    console.error("[Client] Server error:", msg.data);
    ui.showError(msg);
  }
  function handleQuestUpdate(msg) {
    console.log("[Client] Quest update:", msg.data.quest);
  }
  function handleMessage(msg) {
    switch (msg.type) {
      case "welcome":
        handleWelcome(msg);
        break;
      case "playerUpdate":
        handlePlayerUpdate(msg);
        break;
      case "chunkAdd":
        handleChunkAdd(msg);
        break;
      case "chunkRemove":
        handleChunkRemove(msg);
        break;
      case "entityDelta":
        handleEntityDelta(msg);
        break;
      case "sessionRevoked":
        handleSessionRevoked(msg);
        break;
      case "achievement":
        handleAchievement(msg);
        break;
      case "notification":
        handleNotification(msg);
        break;
      case "error":
        handleError(msg);
        break;
      case "questUpdate":
        handleQuestUpdate(msg);
        break;
      default:
        console.log("[Client] Unhandled message:", msg);
    }
  }
  function init() {
    console.log("[Client] Initializing...");
    connection.onMessage(handleMessage);
    connection.onClose((code, reason) => {
      console.log("[Client] Connection closed:", code, reason);
      state.connected = false;
      input.stop();
      renderer.stop();
      world.clear();
      if (ui.getCurrentScreen() !== "session_revoked") {
        ui.showScreen("login");
      }
    });
    input.emitZoom = (delta) => {
      camera.adjustZoom(delta);
    };
    const storedToken = connection.loadStoredToken();
    if (storedToken) {
      console.log("[Client] Found stored token, auto-connecting...");
      connection.connect();
    }
    ui.showScreen("login");
    console.log("[Client] Initialized");
  }
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
