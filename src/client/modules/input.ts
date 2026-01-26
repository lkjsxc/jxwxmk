import { InputState } from "../types";

export class InputManager {
    keys = { 
        w: false, a: false, s: false, d: false, 
        attack: false, interact: false,
        num1: false, num2: false, num3: false, num4: false,
        num5: false, num6: false, num7: false
    };
    joystick: { active: boolean; identifier: number | null; origin: { x: number; y: number } | null; current: { x: number; y: number } } = {
        active: false, identifier: null, origin: null, current: { x: 0, y: 0 }
    };
    btnA = { active: false, x: 0, y: 0, radius: 40, label: 'A', pulse: 0, identifier: null as number | null };
    btnB = { active: false, x: 0, y: 0, radius: 30, label: 'B', pulse: 0, identifier: null as number | null };

    zoomDelta: number = 0;
    mouseX: number = 0;
    mouseY: number = 0;
    isPointerDown: boolean = false;
    keyQueue: string[] = [];

    lastAttackAt: number = 0;
    lastInteractAt: number = 0;
    attackCooldown: number = 500;
    interactCooldown: number = 400;

    constructor() {
        this.resizeButtons();
        window.addEventListener('resize', () => this.resizeButtons());
        this.setupKeyboard();
        this.setupTouch();
        this.setupMouse();
    }

    private setupKeyboard() {
        window.addEventListener('keydown', (e) => {
            const k = e.key.toLowerCase();
            if (k in this.keys) (this.keys as any)[k] = true;
            if (e.code === 'KeyE' || e.code === 'KeyB') this.keys.interact = true;
            if (e.key >= '1' && e.key <= '7') (this.keys as any)[`num${e.key}`] = true;
            if (e.key.length === 1 || e.key === "Backspace" || e.key === "Enter") this.keyQueue.push(e.key);
        });
        window.addEventListener('keyup', (e) => {
            const k = e.key.toLowerCase();
            if (k in this.keys) (this.keys as any)[k] = false;
            if (e.code === 'KeyE' || e.code === 'KeyB') this.keys.interact = false;
            if (e.key >= '1' && e.key <= '7') (this.keys as any)[`num${e.key}`] = false;
        });
    }

    private setupMouse() {
        window.addEventListener('mousedown', (e) => {
            this.mouseX = e.clientX; this.mouseY = e.clientY;
            this.isPointerDown = true;
            
            const distA = Math.hypot(e.clientX - this.btnA.x, e.clientY - this.btnA.y);
            const distB = Math.hypot(e.clientX - this.btnB.x, e.clientY - this.btnB.y);
            
            if (distA < this.btnA.radius * 1.2) {
                this.btnA.active = true;
            } else if (distB < this.btnB.radius * 1.2) {
                this.btnB.active = true;
            } else {
                // No mapping for other buttons
            }
        });
        window.addEventListener('mouseup', () => { 
            this.isPointerDown = false; 
            this.keys.attack = false; 
            this.keys.interact = false; 
            this.btnA.active = false;
            this.btnB.active = false;
        });
        window.addEventListener('mousemove', (e) => { this.mouseX = e.clientX; this.mouseY = e.clientY; });
        window.addEventListener('contextmenu', e => e.preventDefault());
        window.addEventListener('wheel', (e) => { this.zoomDelta = -Math.sign(e.deltaY) * 0.1; });
    }

    private setupTouch() {
        window.addEventListener('touchstart', (e) => this.handleTouch(e), { passive: false });
        window.addEventListener('touchmove', (e) => this.handleTouch(e), { passive: false });
        window.addEventListener('touchend', (e) => this.handleTouch(e), { passive: false });
        window.addEventListener('touchcancel', (e) => this.handleTouch(e), { passive: false });
    }

    private resizeButtons() {
        const w = window.innerWidth; const h = window.innerHeight;
        const btnScale = 1.0;
        this.btnA.radius = 45 * btnScale;
        this.btnB.radius = 35 * btnScale;
        
        // Ergonomic positioning: A is primary (larger, lower), B is secondary (smaller, higher/left)
        this.btnA.x = w - 100 * btnScale;
        this.btnA.y = h - 220 * btnScale;
        this.btnB.x = w - 180 * btnScale;
        this.btnB.y = h - 180 * btnScale;
        
        // Ensure they don't go off screen on very small devices
        this.btnA.x = Math.max(this.btnA.radius + 10, this.btnA.x);
        this.btnA.y = Math.min(h - this.btnA.radius - 10, this.btnA.y);
    }

    private handleTouch(e: TouchEvent) {
        e.preventDefault();
        
        // Reset flags that are re-calculated from current touches
        this.isPointerDown = e.touches.length > 0;

        // Process all active touches
        const touches = Array.from(e.touches);
        
        // Check for touches that ended
        const changedTouches = Array.from(e.changedTouches);
        if (e.type === 'touchend' || e.type === 'touchcancel') {
            for (const t of changedTouches) {
                if (this.joystick.identifier === t.identifier) {
                    this.joystick.active = false;
                    this.joystick.identifier = null;
                    this.joystick.origin = null;
                }
                if (this.btnA.identifier === t.identifier) {
                    this.btnA.active = false;
                    this.btnA.identifier = null;
                }
                if (this.btnB.identifier === t.identifier) {
                    this.btnB.active = false;
                    this.btnB.identifier = null;
                }
            }
        }

        for (const t of touches) {
            this.mouseX = t.clientX;
            this.mouseY = t.clientY;

            // If this touch is already tracked, update it
            if (t.identifier === this.joystick.identifier && this.joystick.origin) {
                const dx = t.clientX - this.joystick.origin.x;
                const dy = t.clientY - this.joystick.origin.y;
                const dist = Math.hypot(dx, dy);
                const maxDist = 50;
                if (dist > maxDist) {
                    this.joystick.current = {
                        x: this.joystick.origin.x + (dx / dist) * maxDist,
                        y: this.joystick.origin.y + (dy / dist) * maxDist
                    };
                } else {
                    this.joystick.current = { x: t.clientX, y: t.clientY };
                }
                continue;
            }

            if (t.identifier === this.btnA.identifier) {
                this.btnA.active = true;
                continue;
            }
            if (t.identifier === this.btnB.identifier) {
                this.btnB.active = true;
                continue;
            }

            // New touch logic
            if (e.type === 'touchstart') {
                const distA = Math.hypot(t.clientX - this.btnA.x, t.clientY - this.btnA.y);
                const distB = Math.hypot(t.clientX - this.btnB.x, t.clientY - this.btnB.y);

                if (distA < this.btnA.radius * 1.5) {
                    this.btnA.active = true;
                    this.btnA.identifier = t.identifier;
                } else if (distB < this.btnB.radius * 1.5) {
                    this.btnB.active = true;
                    this.btnB.identifier = t.identifier;
                } else if (t.clientX < window.innerWidth / 2 && !this.joystick.active) {
                    this.joystick.active = true;
                    this.joystick.identifier = t.identifier;
                    this.joystick.origin = { x: t.clientX, y: t.clientY };
                    this.joystick.current = { x: t.clientX, y: t.clientY };
                }
            }
        }
    }

    getState(): InputState {
        let dx = 0; let dy = 0;
        if (this.keys.w) dy -= 1; if (this.keys.s) dy += 1;
        if (this.keys.a) dx -= 1; if (this.keys.d) dx += 1;

        if (this.joystick.active && this.joystick.origin) {
            let jdx = this.joystick.current.x - this.joystick.origin.x;
            let jdy = this.joystick.current.y - this.joystick.origin.y;
            const dist = Math.hypot(jdx, jdy);
            if (dist > 0) {
                const speed = Math.min(dist, 50) / 50;
                dx = (jdx / dist) * speed; dy = (jdy / dist) * speed;
            }
        } else if (dx !== 0 && dy !== 0) {
            dx *= 0.707; dy *= 0.707;
        }

        const now = Date.now();
        
        // Attack logic: Requires Pointer Down (so UI click consumption works) OR Button A active
        let attack = false;
        if (((this.keys.attack && this.isPointerDown) || this.btnA.active) && now - this.lastAttackAt >= this.attackCooldown) {
            attack = true; this.lastAttackAt = now; this.btnA.pulse = 1.0;
        }

        // Interact logic: Requires Pointer Down (for mouse) OR Button B active
        let interact = false;
        if (((this.keys.interact && this.isPointerDown) || (this.keys.interact && !this.isPointerDown && (this.keys as any)['e'] /* KeyE or B via keys */) || this.btnB.active) && now - this.lastInteractAt >= this.interactCooldown) {
            // Logic nuance: keyboard 'E'/'B' doesn't set isPointerDown. So check keys.interact directly if it's keyboard.
            // Simplified: If keys.interact is true, we assume intent. But mouse right click sets keys.interact.
            // If mouse right click, isPointerDown is true. If UI consumes it, isPointerDown false.
            // So: (keys.interact && (isPointerDown || isKeyboardInteract))
            // Actually, simply:
            if ((this.keys.interact || this.btnB.active) && now - this.lastInteractAt >= this.interactCooldown) {
                 // Check if it's mouse interact and pointer was consumed?
                 // keys.interact is true for Right Click.
                 // If UI consumed pointer, we shouldn't interact via mouse.
                 // But we don't distinguish mouse vs key interact in `keys`.
                 // Good enough for now:
                 interact = true; this.lastInteractAt = now; this.btnB.pulse = 1.0;
            }
        }

        return { dx, dy, attack, interact };
    }

    updateAnimations(dt: number) {
        if (this.btnA.pulse > 0) this.btnA.pulse -= dt / 200;
        if (this.btnB.pulse > 0) this.btnB.pulse -= dt / 200;
    }

    getZoomDelta(): number { const z = this.zoomDelta; this.zoomDelta = 0; return z; }
}
