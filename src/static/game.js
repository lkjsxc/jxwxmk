"use strict";
(() => {
  // core/network.ts
  var NetworkManager = class {
    constructor(token) {
      this.token = token;
      this.ws = null;
      this.handlers = /* @__PURE__ */ new Set();
      this.reconnectAttempts = 0;
      this.maxReconnectAttempts = 5;
    }
    connect() {
      if (!this.token) {
        console.error("No session token available");
        return;
      }
      if (this.ws?.readyState === WebSocket.OPEN) {
        return;
      }
      if (this.ws) {
        this.ws.close();
      }
      const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
      const wsUrl = `${protocol}//${window.location.host}/ws?token=${this.token}`;
      this.ws = new WebSocket(wsUrl);
      this.ws.onopen = () => {
        console.log("WebSocket connected");
        this.reconnectAttempts = 0;
      };
      this.ws.onmessage = (event) => {
        try {
          const msg = JSON.parse(event.data);
          this.handlers.forEach((h) => h(msg));
        } catch (e) {
          console.error("Failed to parse message:", e);
        }
      };
      this.ws.onclose = (event) => {
        console.log("WebSocket closed:", event.code);
        this.ws = null;
      };
      this.ws.onerror = (error) => {
        console.error("WebSocket error:", error);
      };
    }
    disconnect() {
      this.ws?.close();
      this.ws = null;
    }
    send(data) {
      if (this.ws?.readyState === WebSocket.OPEN) {
        this.ws.send(JSON.stringify(data));
        return true;
      }
      return false;
    }
    onMessage(handler) {
      this.handlers.add(handler);
      return () => this.handlers.delete(handler);
    }
    isConnected() {
      return this.ws?.readyState === WebSocket.OPEN;
    }
    static async claimSession() {
      try {
        const response = await fetch("/session/claim", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ player_id: generateUUID() })
        });
        if (!response.ok)
          throw new Error(`HTTP ${response.status}`);
        return await response.json();
      } catch (err) {
        console.error("Claim session error:", err);
        return null;
      }
    }
  };
  function generateUUID() {
    return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, (c) => {
      const r = Math.random() * 16 | 0;
      const v = c === "x" ? r : r & 3 | 8;
      return v.toString(16);
    });
  }

  // core/state.ts
  var GameState = class {
    constructor() {
      this.playerState = null;
      this.chunks = /* @__PURE__ */ new Map();
      this.localPlayerPos = { x: 0, y: 0 };
      this.handlers = /* @__PURE__ */ new Set();
    }
    // Player state
    setPlayerState(state) {
      this.playerState = state;
      this.notify();
    }
    getPlayerState() {
      return this.playerState;
    }
    isSpawned() {
      return this.playerState?.spawned ?? false;
    }
    // Local position tracking for rendering
    setLocalPosition(x, y) {
      this.localPlayerPos.x = x;
      this.localPlayerPos.y = y;
    }
    getLocalPosition() {
      return { ...this.localPlayerPos };
    }
    // Chunk management
    addChunk(chunk) {
      const key = chunkKey(chunk.coord);
      this.chunks.set(key, chunk);
    }
    removeChunk(coord) {
      this.chunks.delete(chunkKey(coord));
    }
    getChunk(coord) {
      return this.chunks.get(chunkKey(coord));
    }
    getAllChunks() {
      return Array.from(this.chunks.values());
    }
    updateEntities(chunkCoord, updates, removes) {
      const chunk = this.getChunk(chunkCoord);
      if (!chunk)
        return;
      for (const entity of updates) {
        chunk.entities.set(entity.id, entity);
      }
      for (const remove of removes) {
        chunk.entities.delete(remove.id);
      }
    }
    getAllEntities() {
      const entities = [];
      for (const chunk of this.chunks.values()) {
        for (const entity of chunk.entities.values()) {
          entities.push(entity);
        }
      }
      return entities;
    }
    // Subscription
    subscribe(handler) {
      this.handlers.add(handler);
      return () => this.handlers.delete(handler);
    }
    notify() {
      this.handlers.forEach((h) => h());
    }
  };
  function chunkKey(coord) {
    return `${coord[0]},${coord[1]}`;
  }

  // render/camera.ts
  var Camera = class {
    constructor() {
      this.x = 0;
      this.y = 0;
      this.zoom = 1.1;
      // Start at 1.1 as per docs
      this.targetX = 0;
      this.targetY = 0;
      // Configuration
      this.lerpFactor = 0.1;
      this.minZoom = 0.75;
      this.maxZoom = 2;
      this.defaultZoom = 1.1;
      this.pixelsPerUnit = 16;
      // Snapping
      this.firstFollow = true;
    }
    follow(targetX, targetY) {
      this.targetX = targetX;
      this.targetY = targetY;
      if (this.firstFollow) {
        this.x = targetX;
        this.y = targetY;
        this.firstFollow = false;
      }
    }
    update() {
      this.x += (this.targetX - this.x) * this.lerpFactor;
      this.y += (this.targetY - this.y) * this.lerpFactor;
    }
    adjustZoom(delta) {
      this.zoom = Math.max(
        this.minZoom,
        Math.min(this.maxZoom, this.zoom + delta)
      );
    }
    resetZoom() {
      this.zoom = this.defaultZoom;
    }
    // Convert world units to screen pixels
    worldToScreen(worldX, worldY, canvasWidth, canvasHeight) {
      const scale = this.pixelsPerUnit * this.zoom;
      return {
        x: (worldX - this.x) * scale + canvasWidth / 2,
        y: (worldY - this.y) * scale + canvasHeight / 2
      };
    }
    // Convert screen pixels to world units
    screenToWorld(screenX, screenY, canvasWidth, canvasHeight) {
      const scale = this.pixelsPerUnit * this.zoom;
      return {
        x: (screenX - canvasWidth / 2) / scale + this.x,
        y: (screenY - canvasHeight / 2) / scale + this.y
      };
    }
    getScale() {
      return this.pixelsPerUnit * this.zoom;
    }
    reset() {
      this.firstFollow = true;
      this.zoom = this.defaultZoom;
    }
  };

  // render/renderer.ts
  var Renderer = class {
    constructor(canvas, camera, state) {
      this.canvas = canvas;
      const ctx = canvas.getContext("2d");
      if (!ctx)
        throw new Error("Failed to get canvas context");
      this.ctx = ctx;
      this.camera = camera;
      this.state = state;
      this.setupResize();
    }
    setupResize() {
      const resize = () => {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
      };
      resize();
      window.addEventListener("resize", resize);
    }
    render() {
      const ctx = this.ctx;
      const canvas = this.canvas;
      ctx.fillStyle = "#0f0f1e";
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      this.drawGrid();
      this.drawWorld();
      this.drawJoystick();
    }
    drawGrid() {
      const ctx = this.ctx;
      const scale = this.camera.getScale();
      const gridSize = 16 * scale;
      const offsetX = (-this.camera.x * scale + this.canvas.width / 2) % gridSize;
      const offsetY = (-this.camera.y * scale + this.canvas.height / 2) % gridSize;
      ctx.strokeStyle = "rgba(100, 100, 150, 0.15)";
      ctx.lineWidth = 1;
      ctx.beginPath();
      for (let x = offsetX; x < this.canvas.width; x += gridSize) {
        ctx.moveTo(x, 0);
        ctx.lineTo(x, this.canvas.height);
      }
      for (let y = offsetY; y < this.canvas.height; y += gridSize) {
        ctx.moveTo(0, y);
        ctx.lineTo(this.canvas.width, y);
      }
      ctx.stroke();
    }
    drawWorld() {
      const player = this.state.getPlayerState();
      if (!player?.spawned)
        return;
      const entities = this.state.getAllEntities();
      entities.sort((a, b) => a.y - b.y);
      for (const entity of entities) {
        this.drawEntity(entity);
      }
      this.drawLocalPlayer();
    }
    drawEntity(entity) {
      const ctx = this.ctx;
      const pos = this.camera.worldToScreen(
        entity.x,
        entity.y,
        this.canvas.width,
        this.canvas.height
      );
      const scale = this.camera.getScale();
      if (pos.x < -50 || pos.x > this.canvas.width + 50 || pos.y < -50 || pos.y > this.canvas.height + 50) {
        return;
      }
      const radius = 8 * (this.camera.zoom / 1.1);
      switch (entity.kind) {
        case "player":
          this.drawCircle(pos.x, pos.y, radius, "#6a6aff");
          if (entity.name) {
            this.drawLabel(pos.x, pos.y - radius - 5, entity.name);
          }
          break;
        case "resource":
          this.drawResource(entity.subtype || "", pos.x, pos.y, radius, scale);
          break;
        case "mob":
          this.drawCircle(pos.x, pos.y, radius, "#ff6a6a");
          if (entity.name) {
            this.drawLabel(pos.x, pos.y - radius - 5, entity.name);
          }
          break;
        case "structure":
          this.drawRect(pos.x - radius, pos.y - radius, radius * 2, radius * 2, "#8a8aaa");
          break;
        case "npc":
          this.drawCircle(pos.x, pos.y, radius, "#ffaa44");
          if (entity.name) {
            this.drawLabel(pos.x, pos.y - radius - 5, entity.name);
          }
          break;
      }
      if (entity.hp !== void 0 && entity.max_hp !== void 0 && entity.hp < entity.max_hp) {
        this.drawHPBar(pos.x, pos.y - radius - 12, radius * 2, 4, entity.hp, entity.max_hp);
      }
    }
    drawResource(subtype, x, y, radius, scale) {
      const colors = {
        tree: "#4a8a4a",
        stone: "#8a8a8a",
        bush: "#6aaa4a",
        ore: "#6a6a8a"
      };
      const color = colors[subtype] || "#aaaaaa";
      if (subtype === "tree") {
        this.drawRect(x - radius * 0.5, y - radius * 1.5, radius, radius * 2.5, color);
      } else {
        this.drawCircle(x, y, radius, color);
      }
    }
    drawLocalPlayer() {
      const pos = this.camera.worldToScreen(
        this.camera.x,
        this.camera.y,
        this.canvas.width,
        this.canvas.height
      );
      const radius = 10 * (this.camera.zoom / 1.1);
      this.drawCircle(pos.x, pos.y, radius, "#6a6aff");
      this.ctx.strokeStyle = "#ffffff";
      this.ctx.lineWidth = 2;
      this.ctx.beginPath();
      this.ctx.arc(pos.x, pos.y, radius + 2, 0, Math.PI * 2);
      this.ctx.stroke();
    }
    drawCircle(x, y, radius, color) {
      this.ctx.fillStyle = color;
      this.ctx.beginPath();
      this.ctx.arc(x, y, radius, 0, Math.PI * 2);
      this.ctx.fill();
    }
    drawRect(x, y, width, height, color) {
      this.ctx.fillStyle = color;
      this.ctx.fillRect(x, y, width, height);
    }
    drawLabel(x, y, text) {
      this.ctx.fillStyle = "#ffffff";
      this.ctx.font = "10px sans-serif";
      this.ctx.textAlign = "center";
      this.ctx.fillText(text, x, y);
    }
    drawHPBar(x, y, width, height, hp, maxHP) {
      const pct = hp / maxHP;
      this.ctx.fillStyle = "#333";
      this.ctx.fillRect(x - width / 2, y, width, height);
      this.ctx.fillStyle = pct > 0.5 ? "#4a4" : pct > 0.25 ? "#aa4" : "#a44";
      this.ctx.fillRect(x - width / 2, y, width * pct, height);
    }
    drawJoystick() {
    }
  };

  // input/keyboard.ts
  var KeyboardManager = class {
    constructor() {
      this.keys = /* @__PURE__ */ new Set();
      this.handlers = /* @__PURE__ */ new Set();
      this.setupListeners();
    }
    setupListeners() {
      window.addEventListener("keydown", (e) => {
        if (e.repeat)
          return;
        this.keys.add(e.key);
        this.handlers.forEach((h) => h(e.key, true));
      });
      window.addEventListener("keyup", (e) => {
        this.keys.delete(e.key);
        this.handlers.forEach((h) => h(e.key, false));
      });
    }
    isPressed(key) {
      return this.keys.has(key);
    }
    getMovementVector() {
      let dx = 0;
      let dy = 0;
      if (this.isPressed("w") || this.isPressed("W") || this.isPressed("ArrowUp"))
        dy -= 1;
      if (this.isPressed("s") || this.isPressed("S") || this.isPressed("ArrowDown"))
        dy += 1;
      if (this.isPressed("a") || this.isPressed("A") || this.isPressed("ArrowLeft"))
        dx -= 1;
      if (this.isPressed("d") || this.isPressed("D") || this.isPressed("ArrowRight"))
        dx += 1;
      if (dx !== 0 || dy !== 0) {
        const mag = Math.sqrt(dx * dx + dy * dy);
        dx /= mag;
        dy /= mag;
      }
      return { dx, dy };
    }
    onKey(handler) {
      this.handlers.add(handler);
      return () => this.handlers.delete(handler);
    }
    destroy() {
    }
  };

  // input/touch.ts
  var LONG_PRESS_THRESHOLD = 250;
  var JOYSTICK_MAX_RADIUS = 50;
  var TouchManager = class {
    constructor(joystickEl, joystickZone, actionZone) {
      this.joystickState = {
        active: false,
        touchId: null,
        centerX: 0,
        centerY: 0,
        dx: 0,
        dy: 0
      };
      this.pointerState = {
        x: 0,
        y: 0,
        down: false,
        startTime: 0,
        longPressTriggered: false,
        touchId: null
      };
      this.joystickHandlers = /* @__PURE__ */ new Set();
      this.gestureHandlers = /* @__PURE__ */ new Set();
      this.longPressTimer = null;
      this.onJoystickStart = (e) => {
        e.preventDefault();
        if (this.joystickState.active)
          return;
        const touch = e.changedTouches[0];
        this.joystickState.active = true;
        this.joystickState.touchId = touch.identifier;
        this.joystickState.centerX = touch.clientX;
        this.joystickState.centerY = touch.clientY;
        this.joystickState.dx = 0;
        this.joystickState.dy = 0;
        this.updateJoystickVisual();
        this.joystickEl.classList.remove("hidden");
      };
      this.onJoystickMove = (e) => {
        e.preventDefault();
        if (!this.joystickState.active)
          return;
        const touch = this.findTouch(e.changedTouches, this.joystickState.touchId);
        if (!touch)
          return;
        let dx = touch.clientX - this.joystickState.centerX;
        let dy = touch.clientY - this.joystickState.centerY;
        const distance = Math.sqrt(dx * dx + dy * dy);
        if (distance > JOYSTICK_MAX_RADIUS) {
          const ratio = JOYSTICK_MAX_RADIUS / distance;
          dx *= ratio;
          dy *= ratio;
        }
        this.joystickState.dx = dx / JOYSTICK_MAX_RADIUS;
        this.joystickState.dy = dy / JOYSTICK_MAX_RADIUS;
        this.updateJoystickVisual();
        this.notifyJoystick();
      };
      this.onJoystickEnd = (e) => {
        e.preventDefault();
        const touch = this.findTouch(e.changedTouches, this.joystickState.touchId);
        if (!touch)
          return;
        this.joystickState.active = false;
        this.joystickState.touchId = null;
        this.joystickState.dx = 0;
        this.joystickState.dy = 0;
        this.joystickEl.classList.add("hidden");
        this.notifyJoystick();
      };
      this.onActionStart = (e) => {
        e.preventDefault();
        if (this.pointerState.down)
          return;
        const touch = e.changedTouches[0];
        this.pointerState.down = true;
        this.pointerState.touchId = touch.identifier;
        this.pointerState.x = touch.clientX;
        this.pointerState.y = touch.clientY;
        this.pointerState.startTime = Date.now();
        this.pointerState.longPressTriggered = false;
        this.longPressTimer = window.setTimeout(() => {
          if (this.pointerState.down && !this.pointerState.longPressTriggered) {
            this.pointerState.longPressTriggered = true;
            this.notifyGesture("longpress", this.pointerState.x, this.pointerState.y);
          }
        }, LONG_PRESS_THRESHOLD);
      };
      this.onActionMove = (e) => {
        e.preventDefault();
        if (!this.pointerState.down)
          return;
        const touch = this.findTouch(e.changedTouches, this.pointerState.touchId);
        if (!touch)
          return;
        this.pointerState.x = touch.clientX;
        this.pointerState.y = touch.clientY;
      };
      this.onActionEnd = (e) => {
        e.preventDefault();
        const touch = this.findTouch(e.changedTouches, this.pointerState.touchId);
        if (!touch)
          return;
        if (this.longPressTimer) {
          clearTimeout(this.longPressTimer);
          this.longPressTimer = null;
        }
        if (this.pointerState.down && !this.pointerState.longPressTriggered) {
          const duration = Date.now() - this.pointerState.startTime;
          if (duration < LONG_PRESS_THRESHOLD) {
            this.notifyGesture("tap", this.pointerState.x, this.pointerState.y);
          }
        }
        this.pointerState.down = false;
        this.pointerState.touchId = null;
      };
      this.joystickEl = joystickEl;
      this.joystickZone = joystickZone;
      this.actionZone = actionZone;
      this.setupListeners();
    }
    setupListeners() {
      this.joystickZone.addEventListener("touchstart", this.onJoystickStart, { passive: false });
      this.joystickZone.addEventListener("touchmove", this.onJoystickMove, { passive: false });
      this.joystickZone.addEventListener("touchend", this.onJoystickEnd, { passive: false });
      this.joystickZone.addEventListener("touchcancel", this.onJoystickEnd, { passive: false });
      this.actionZone.addEventListener("touchstart", this.onActionStart, { passive: false });
      this.actionZone.addEventListener("touchmove", this.onActionMove, { passive: false });
      this.actionZone.addEventListener("touchend", this.onActionEnd, { passive: false });
      this.actionZone.addEventListener("touchcancel", this.onActionEnd, { passive: false });
    }
    findTouch(touches, id) {
      if (id === null)
        return null;
      for (let i = 0; i < touches.length; i++) {
        if (touches[i].identifier === id) {
          return touches[i];
        }
      }
      return null;
    }
    updateJoystickVisual() {
      const scale = JOYSTICK_MAX_RADIUS;
      const knobX = this.joystickState.dx * scale;
      const knobY = this.joystickState.dy * scale;
      this.joystickEl.style.left = `${this.joystickState.centerX}px`;
      this.joystickEl.style.top = `${this.joystickState.centerY}px`;
      const knob = this.joystickEl.querySelector(".joystick-knob");
      if (knob) {
        knob.style.transform = `translate(calc(-50% + ${knobX}px), calc(-50% + ${knobY}px))`;
      }
    }
    notifyJoystick() {
      this.joystickHandlers.forEach((h) => h(this.joystickState.dx, this.joystickState.dy));
    }
    notifyGesture(type, x, y) {
      this.gestureHandlers.forEach((h) => h(type, x, y));
    }
    getJoystickVector() {
      if (!this.joystickState.active)
        return { dx: 0, dy: 0 };
      return { dx: this.joystickState.dx, dy: this.joystickState.dy };
    }
    isJoystickActive() {
      return this.joystickState.active;
    }
    onJoystick(handler) {
      this.joystickHandlers.add(handler);
      return () => this.joystickHandlers.delete(handler);
    }
    onGesture(handler) {
      this.gestureHandlers.add(handler);
      return () => this.gestureHandlers.delete(handler);
    }
    getPointerPosition() {
      if (!this.pointerState.down)
        return null;
      return { x: this.pointerState.x, y: this.pointerState.y };
    }
    destroy() {
      if (this.longPressTimer) {
        clearTimeout(this.longPressTimer);
      }
    }
  };

  // input/mouse.ts
  var LONG_PRESS_THRESHOLD2 = 250;
  var MouseManager = class {
    constructor(canvas, camera) {
      this.canvas = canvas;
      this.camera = camera;
      this.x = 0;
      this.y = 0;
      this.down = false;
      this.startTime = 0;
      this.longPressTriggered = false;
      this.longPressTimer = null;
      this.handlers = /* @__PURE__ */ new Set();
      this.wheelHandlers = /* @__PURE__ */ new Set();
      this.onMouseDown = (e) => {
        this.down = true;
        this.x = e.clientX;
        this.y = e.clientY;
        this.startTime = Date.now();
        this.longPressTriggered = false;
        this.handlers.forEach((h) => h("down", this.x, this.y));
        this.longPressTimer = window.setTimeout(() => {
          if (this.down && !this.longPressTriggered) {
            this.longPressTriggered = true;
            this.handlers.forEach((h) => h("up", this.x, this.y));
          }
        }, LONG_PRESS_THRESHOLD2);
      };
      this.onMouseUp = (e) => {
        if (!this.down)
          return;
        if (this.longPressTimer) {
          clearTimeout(this.longPressTimer);
          this.longPressTimer = null;
        }
        this.down = false;
        this.handlers.forEach((h) => h("up", e.clientX, e.clientY));
      };
      this.onMouseMove = (e) => {
        this.x = e.clientX;
        this.y = e.clientY;
        this.handlers.forEach((h) => h("move", this.x, this.y));
      };
      this.handleWheel = (e) => {
        e.preventDefault();
        const delta = e.deltaY > 0 ? -0.1 : 0.1;
        this.wheelHandlers.forEach((h) => h(delta));
      };
      this.setupListeners();
    }
    setupListeners() {
      this.canvas.addEventListener("mousedown", this.onMouseDown);
      this.canvas.addEventListener("mouseup", this.onMouseUp);
      this.canvas.addEventListener("mousemove", this.onMouseMove);
      this.canvas.addEventListener("wheel", this.handleWheel, { passive: false });
      this.canvas.addEventListener("contextmenu", (e) => e.preventDefault());
    }
    getPosition() {
      return { x: this.x, y: this.y };
    }
    isDown() {
      return this.down;
    }
    getWorldPosition(canvasWidth, canvasHeight) {
      return this.camera.screenToWorld(this.x, this.y, canvasWidth, canvasHeight);
    }
    onMouse(handler) {
      this.handlers.add(handler);
      return () => this.handlers.delete(handler);
    }
    onWheel(handler) {
      this.wheelHandlers.add(handler);
      return () => this.wheelHandlers.delete(handler);
    }
    destroy() {
      if (this.longPressTimer) {
        clearTimeout(this.longPressTimer);
      }
      this.canvas.removeEventListener("mousedown", this.onMouseDown);
      this.canvas.removeEventListener("mouseup", this.onMouseUp);
      this.canvas.removeEventListener("mousemove", this.onMouseMove);
      this.canvas.removeEventListener("wheel", this.handleWheel);
    }
  };

  // input/manager.ts
  var INPUT_INTERVAL = 50;
  var ATTACK_COOLDOWN = 500;
  var INTERACT_COOLDOWN = 400;
  var InputManager = class {
    constructor(network, camera, keyboard, touch, mouse, canvas, isMenuOpen) {
      this.network = network;
      this.camera = camera;
      this.keyboard = keyboard;
      this.touch = touch;
      this.mouse = mouse;
      this.canvas = canvas;
      this.isMenuOpen = isMenuOpen;
      this.interval = null;
      this.lastAttackTime = 0;
      this.lastInteractTime = 0;
      this.setupInputs();
      this.startInputLoop();
    }
    setupInputs() {
      this.touch.onGesture((type, x, y) => {
        if (this.isMenuOpen())
          return;
        const worldPos = this.camera.screenToWorld(
          x,
          y,
          this.canvas.width,
          this.canvas.height
        );
        if (type === "tap") {
          this.sendAttack(worldPos.x, worldPos.y);
        } else if (type === "longpress") {
          this.sendInteract(worldPos.x, worldPos.y);
        }
      });
      this.mouse.onMouse((type, x, y) => {
        if (this.isMenuOpen())
          return;
        if (type === "up") {
          const worldPos = this.camera.screenToWorld(
            x,
            y,
            this.canvas.width,
            this.canvas.height
          );
          this.sendAttack(worldPos.x, worldPos.y);
        }
      });
      this.mouse.onWheel((delta) => {
        this.camera.adjustZoom(delta);
      });
    }
    sendAttack(targetX, targetY) {
      const now = Date.now();
      if (now - this.lastAttackTime < ATTACK_COOLDOWN)
        return;
      this.lastAttackTime = now;
      this.network.send({
        type: "input",
        data: {
          dx: 0,
          dy: 0,
          attack: true,
          interact: false,
          aim: { x: targetX, y: targetY }
        }
      });
    }
    sendInteract(targetX, targetY) {
      const now = Date.now();
      if (now - this.lastInteractTime < INTERACT_COOLDOWN)
        return;
      this.lastInteractTime = now;
      this.network.send({
        type: "input",
        data: {
          dx: 0,
          dy: 0,
          attack: false,
          interact: true,
          aim: { x: targetX, y: targetY }
        }
      });
    }
    startInputLoop() {
      this.interval = window.setInterval(() => {
        if (this.isMenuOpen())
          return;
        const keyboardVec = this.keyboard.getMovementVector();
        const joystickVec = this.touch.getJoystickVector();
        let dx = 0;
        let dy = 0;
        if (this.touch.isJoystickActive()) {
          dx = joystickVec.dx;
          dy = joystickVec.dy;
        } else if (keyboardVec.dx !== 0 || keyboardVec.dy !== 0) {
          dx = keyboardVec.dx;
          dy = keyboardVec.dy;
        }
        if (dx !== 0 || dy !== 0) {
          this.network.send({
            type: "input",
            data: {
              dx,
              dy,
              attack: false,
              interact: false,
              aim: null
            }
          });
        }
      }, INPUT_INTERVAL);
    }
    destroy() {
      if (this.interval) {
        clearInterval(this.interval);
      }
    }
  };

  // ui/hud.ts
  var HUDManager = class {
    constructor() {
      // Callback for slot selection
      this.onSlotSelect = null;
      this.hpBar = document.getElementById("hp-bar");
      this.hungerBar = document.getElementById("hunger-bar");
      this.tempBar = document.getElementById("temp-bar");
      this.hotbarSlots = document.querySelectorAll(".hotbar-slot");
      this.activeItemName = document.getElementById("active-item-name");
      this.setupHotbarClicks();
    }
    setupHotbarClicks() {
      this.hotbarSlots.forEach((slot, index) => {
        slot.addEventListener("click", () => {
          this.onSlotSelect?.(index);
        });
      });
    }
    update(playerState) {
      if (this.hpBar) {
        const pct = playerState.vitals.hp / playerState.vitals.max_hp * 100;
        this.hpBar.style.width = `${pct}%`;
      }
      if (this.hungerBar) {
        const pct = playerState.vitals.hunger / playerState.vitals.max_hunger * 100;
        this.hungerBar.style.width = `${pct}%`;
      }
      if (this.tempBar) {
        const pct = playerState.vitals.temperature / playerState.vitals.max_temperature * 100;
        this.tempBar.style.width = `${pct}%`;
      }
      this.updateHotbar(playerState.active_slot, playerState.inventory);
    }
    updateHotbar(activeSlot, inventory) {
      this.hotbarSlots.forEach((slot, index) => {
        slot.classList.toggle("active", index === activeSlot);
        const item = inventory[index];
        if (item) {
          slot.textContent = item.count > 1 ? `${item.count}` : "";
          slot.setAttribute("title", item.item);
        } else {
          slot.textContent = (index + 1).toString();
          slot.removeAttribute("title");
        }
      });
      const activeItem = inventory[activeSlot];
      if (this.activeItemName) {
        this.activeItemName.textContent = activeItem ? activeItem.item : "";
      }
    }
    show() {
      document.getElementById("hud")?.classList.remove("hidden");
    }
    hide() {
      document.getElementById("hud")?.classList.add("hidden");
    }
  };

  // ui/inventory.ts
  var InventoryManager = class {
    constructor() {
      this.isOpen = false;
      // Callback for slot selection
      this.onSlotSelect = null;
      this.modal = document.getElementById("inventory-modal");
      this.grid = document.getElementById("inventory-grid");
      this.levelEl = document.getElementById("inv-level");
      this.xpEl = document.getElementById("inv-xp");
      this.killsEl = document.getElementById("inv-kills");
      this.craftsEl = document.getElementById("inv-crafts");
      this.deathsEl = document.getElementById("inv-deaths");
      this.setupListeners();
    }
    setupListeners() {
      document.getElementById("close-inventory")?.addEventListener("click", () => {
        this.close();
      });
    }
    toggle() {
      if (this.isOpen) {
        this.close();
      } else {
        this.open();
      }
    }
    open() {
      this.modal?.classList.remove("hidden");
      this.isOpen = true;
    }
    close() {
      this.modal?.classList.add("hidden");
      this.isOpen = false;
    }
    isOpened() {
      return this.isOpen;
    }
    render(playerState) {
      if (!this.grid)
        return;
      this.grid.innerHTML = "";
      playerState.inventory.forEach((slot, index) => {
        const slotEl = document.createElement("div");
        const isActive = index === playerState.active_slot;
        slotEl.className = `inv-slot ${isActive ? "active" : ""}`;
        if (slot) {
          slotEl.innerHTML = `
          <div class="item-icon">${slot.item.charAt(0).toUpperCase()}</div>
          <div class="item-count">${slot.count}</div>
          <div class="item-name">${slot.item}</div>
        `;
        } else {
          slotEl.classList.add("empty");
        }
        slotEl.addEventListener("click", () => {
          this.onSlotSelect?.(index);
        });
        this.grid.appendChild(slotEl);
      });
      if (this.levelEl)
        this.levelEl.textContent = playerState.level.toString();
      if (this.xpEl)
        this.xpEl.textContent = playerState.xp.toString();
      if (this.killsEl)
        this.killsEl.textContent = playerState.stats.kills.toString();
      if (this.craftsEl)
        this.craftsEl.textContent = playerState.stats.crafts.toString();
      if (this.deathsEl)
        this.deathsEl.textContent = playerState.stats.deaths.toString();
    }
  };

  // ui/menu.ts
  var MenuManager = class {
    constructor() {
      this.currentPage = null;
      // Button callbacks
      this.onResume = null;
      this.onInventory = null;
      this.onCrafting = null;
      this.onSettings = null;
      this.onDisconnect = null;
      this.menuModal = document.getElementById("pause-menu");
      this.inventoryModal = document.getElementById("inventory-modal");
      this.craftingModal = document.getElementById("crafting-modal");
      this.settingsModal = document.getElementById("settings-modal");
      this.setupListeners();
      this.createModals();
      this.setupMenuButton();
    }
    setupMenuButton() {
      const menuBtn = document.getElementById("menu-btn");
      if (menuBtn) {
        menuBtn.addEventListener("click", () => {
          this.toggle();
        });
      }
    }
    createModals() {
      if (!this.craftingModal) {
        this.craftingModal = this.createModal("crafting-modal", "Crafting", `
        <div id="crafting-grid" class="crafting-grid">
          <div class="crafting-placeholder">Crafting recipes will appear here</div>
        </div>
      `);
        document.getElementById("ui-layer")?.appendChild(this.craftingModal);
      }
      if (!this.settingsModal) {
        this.settingsModal = this.createModal("settings-modal", "Settings", `
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
        document.getElementById("ui-layer")?.appendChild(this.settingsModal);
      }
    }
    createModal(id, title, content) {
      const modal = document.createElement("div");
      modal.id = id;
      modal.className = "modal hidden";
      modal.innerHTML = `
      <div class="modal-content">
        <div class="modal-header">
          <h2>${title}</h2>
          <button class="btn-close" data-close="${id}">&times;</button>
        </div>
        ${content}
      </div>
    `;
      modal.querySelector(".btn-close")?.addEventListener("click", () => {
        this.close();
      });
      return modal;
    }
    setupListeners() {
      document.getElementById("btn-resume")?.addEventListener("click", () => {
        this.close();
        this.onResume?.();
      });
      document.getElementById("btn-inventory")?.addEventListener("click", () => {
        this.showPage("inventory");
        this.onInventory?.();
      });
      document.getElementById("btn-crafting")?.addEventListener("click", () => {
        this.showPage("crafting");
        this.onCrafting?.();
      });
      document.getElementById("btn-settings")?.addEventListener("click", () => {
        this.showPage("settings");
        this.onSettings?.();
      });
      document.getElementById("btn-disconnect")?.addEventListener("click", () => {
        this.close();
        this.onDisconnect?.();
      });
      document.getElementById("close-menu")?.addEventListener("click", () => {
        this.close();
      });
      window.addEventListener("keydown", (e) => {
        if (e.key === "Escape") {
          if (this.currentPage && this.currentPage !== "menu") {
            this.showPage("menu");
          } else {
            this.toggle();
          }
        }
      });
    }
    toggle() {
      if (this.currentPage) {
        this.close();
      } else {
        this.showPage("menu");
      }
    }
    showPage(page) {
      this.hideAllModals();
      this.currentPage = page;
      switch (page) {
        case "menu":
          this.menuModal?.classList.remove("hidden");
          break;
        case "inventory":
          this.inventoryModal?.classList.remove("hidden");
          break;
        case "crafting":
          this.craftingModal?.classList.remove("hidden");
          break;
        case "settings":
          this.settingsModal?.classList.remove("hidden");
          break;
      }
    }
    close() {
      this.hideAllModals();
      this.currentPage = null;
    }
    hideAllModals() {
      this.menuModal?.classList.add("hidden");
      this.inventoryModal?.classList.add("hidden");
      this.craftingModal?.classList.add("hidden");
      this.settingsModal?.classList.add("hidden");
    }
    isOpen() {
      return this.currentPage !== null;
    }
    getCurrentPage() {
      return this.currentPage;
    }
  };

  // ui/notifications.ts
  var NotificationManager = class {
    constructor() {
      this.container = document.getElementById("notifications");
    }
    show(text, duration = 3e3) {
      if (!this.container)
        return;
      const notif = document.createElement("div");
      notif.className = "notification";
      notif.textContent = text;
      this.container.appendChild(notif);
      setTimeout(() => {
        notif.style.opacity = "0";
        notif.style.transform = "translateX(100%)";
        setTimeout(() => notif.remove(), 300);
      }, duration);
    }
  };

  // ui/login.ts
  var LoginManager = class {
    constructor(callbacks) {
      this.callbacks = callbacks;
      this.loginScreen = document.getElementById("login-screen");
      this.sessionInfo = document.getElementById("session-info");
      this.tokenEl = document.getElementById("token");
      this.setupListeners();
      this.loadSession();
    }
    setupListeners() {
      document.getElementById("claim-session")?.addEventListener("click", () => {
        this.callbacks.onClaimSession();
      });
      document.getElementById("connect")?.addEventListener("click", () => {
        this.callbacks.onConnect();
      });
    }
    loadSession() {
      const token = localStorage.getItem("sessionToken");
      const playerId = localStorage.getItem("playerId");
      if (token && playerId) {
        this.sessionInfo?.classList.remove("hidden");
        if (this.tokenEl) {
          this.tokenEl.textContent = token.substring(0, 8) + "...";
        }
      }
    }
    showSessionInfo(token) {
      this.sessionInfo?.classList.remove("hidden");
      if (this.tokenEl) {
        this.tokenEl.textContent = token.substring(0, 8) + "...";
      }
    }
    hide() {
      this.loginScreen?.classList.add("hidden");
    }
    show() {
      this.loginScreen?.classList.remove("hidden");
    }
  };

  // index.ts
  var GameClient = class {
    constructor() {
      // Core
      this.network = null;
      this.input = null;
      const canvas = document.getElementById("game-canvas");
      if (!canvas)
        throw new Error("Canvas not found");
      this.canvas = canvas;
      this.state = new GameState();
      this.camera = new Camera();
      this.renderer = new Renderer(this.canvas, this.camera, this.state);
      this.keyboard = new KeyboardManager();
      this.touch = new TouchManager(
        document.getElementById("joystick"),
        document.getElementById("joystick-zone"),
        document.getElementById("action-zone")
      );
      this.mouse = new MouseManager(this.canvas, this.camera);
      this.menu = new MenuManager();
      this.hud = new HUDManager();
      this.inventory = new InventoryManager();
      this.notifications = new NotificationManager();
      this.login = new LoginManager({
        onClaimSession: () => this.claimSession(),
        onConnect: () => this.connect()
      });
      this.setupUIHandlers();
      this.setupKeyboardShortcuts();
      this.startRenderLoop();
    }
    setupUIHandlers() {
      this.hud.onSlotSelect = (slot) => {
        this.network?.send({ type: "slot", data: { slot } });
      };
      this.inventory.onSlotSelect = (slot) => {
        this.network?.send({ type: "slot", data: { slot } });
      };
      this.menu.onResume = () => {
      };
      this.menu.onInventory = () => {
        const player = this.state.getPlayerState();
        if (player)
          this.inventory.render(player);
      };
      this.menu.onCrafting = () => {
      };
      this.menu.onSettings = () => {
      };
      this.menu.onDisconnect = () => {
        this.disconnect();
      };
    }
    setupKeyboardShortcuts() {
      this.keyboard.onKey((key, pressed) => {
        if (!pressed)
          return;
        switch (key.toLowerCase()) {
          case "i":
          case "e":
            if (!this.menu.isOpen()) {
              this.inventory.toggle();
              const player = this.state.getPlayerState();
              if (player && this.inventory.isOpened()) {
                this.inventory.render(player);
              }
            }
            break;
          case "escape":
            if (this.inventory.isOpened()) {
              this.inventory.close();
            }
            break;
          case "1":
          case "2":
          case "3":
          case "4":
          case "5":
          case "6":
          case "7": {
            const slot = parseInt(key) - 1;
            this.network?.send({ type: "slot", data: { slot } });
            break;
          }
          case " ":
            if (this.state.isSpawned() && !this.inventory.isOpened() && !this.menu.isOpen()) {
              this.network?.send({ type: "attack", data: { target_id: "nearest" } });
            }
            break;
        }
      });
    }
    async claimSession() {
      const result = await NetworkManager.claimSession();
      if (result) {
        localStorage.setItem("playerId", result.id);
        localStorage.setItem("sessionToken", result.token);
        this.login.showSessionInfo(result.token);
        this.notifications.show("Session claimed!");
      } else {
        this.notifications.show("Failed to claim session");
      }
    }
    connect() {
      const token = localStorage.getItem("sessionToken");
      if (!token) {
        this.notifications.show("No session token");
        return;
      }
      this.network = new NetworkManager(token);
      this.network.onMessage((msg) => this.handleMessage(msg));
      this.network.connect();
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
    disconnect() {
      this.input?.destroy();
      this.input = null;
      this.network?.disconnect();
      this.network = null;
      this.hud.hide();
      this.login.show();
      this.camera.reset();
    }
    handleMessage(msg) {
      switch (msg.type) {
        case "welcome":
          if (!msg.spawned) {
            this.network?.send({ type: "spawn", data: { settlement_id: null } });
          }
          break;
        case "playerUpdate":
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
        case "chunkAdd":
          this.state.addChunk({
            coord: msg.data.coord,
            biome: msg.data.biome,
            entities: new Map([
              ...Object.entries(msg.data.entities.resources),
              ...Object.entries(msg.data.entities.mobs),
              ...Object.entries(msg.data.entities.structures),
              ...Object.entries(msg.data.entities.npcs)
            ])
          });
          break;
        case "chunkRemove":
          this.state.removeChunk(msg.data.coord);
          break;
        case "entityDelta":
          this.state.updateEntities(
            msg.data.chunk,
            msg.data.updates,
            msg.data.removes
          );
          break;
        case "notification":
          this.notifications.show(msg.data.text);
          break;
        case "error":
          this.notifications.show(`Error: ${msg.data.message}`);
          break;
        case "sessionRevoked":
          this.notifications.show("Session revoked");
          localStorage.removeItem("sessionToken");
          localStorage.removeItem("playerId");
          this.disconnect();
          break;
        case "combatResult": {
          const crit = msg.data.critical ? " CRITICAL!" : "";
          this.notifications.show(`Hit: ${msg.data.damage.toFixed(1)} dmg${crit}`);
          break;
        }
        case "resourceDepleted": {
          const items = msg.data.items_received.map((i) => `${i.count}x ${i.item}`).join(", ");
          this.notifications.show(`Gathered: ${items}`);
          break;
        }
        case "achievement":
          this.notifications.show(`Achievement: ${msg.data.name}!`);
          break;
        case "npcInteraction":
          this.notifications.show(`${msg.data.name}: ${msg.data.text}`);
          break;
        case "questUpdate":
          this.notifications.show(`Quest ${msg.data.quest.state}: ${msg.data.quest.name}`);
          break;
      }
    }
    startRenderLoop() {
      const loop = () => {
        this.camera.update();
        this.renderer.render();
        requestAnimationFrame(loop);
      };
      requestAnimationFrame(loop);
    }
  };
  new GameClient();
})();
