import { InputState } from "../types";

export class InputManager {
    keys = { 
        w: false, a: false, s: false, d: false, 
        attack: false, interact: false,
        num1: false, num2: false, num3: false, num4: false,
        num5: false, num6: false, num7: false
    };
    joystick: { active: boolean; origin: { x: number; y: number } | null; current: { x: number; y: number } } = {
        active: false, origin: null, current: { x: 0, y: 0 }
    };
    btnA = { active: false, x: 0, y: 0, radius: 35, label: 'A', pulse: 0 };
    btnB = { active: false, x: 0, y: 0, radius: 25, label: 'B', pulse: 0 };

    zoomDelta: number = 0;
    mouseX: number = 0;
    mouseY: number = 0;
    isPointerDown: boolean = false;

    // Cooldowns
    lastAttackAt: number = 0;
    lastInteractAt: number = 0;
    attackCooldown: number = 500;
    interactCooldown: number = 300;

    constructor() {
        this.setupKeyboard();
        this.setupTouch();
        this.setupMouse();
    }

    private setupKeyboard() {
        window.addEventListener('keydown', (e) => {
            const k = e.key.toLowerCase();
            if (k in this.keys) (this.keys as any)[k] = true;
            if (e.code === 'KeyE') this.keys.interact = true;
            if (e.key >= '1' && e.key <= '7') (this.keys as any)[`num${e.key}`] = true;
        });
        window.addEventListener('keyup', (e) => {
            const k = e.key.toLowerCase();
            if (k in this.keys) (this.keys as any)[k] = false;
            if (e.code === 'KeyE') this.keys.interact = false;
            if (e.key >= '1' && e.key <= '7') (this.keys as any)[`num${e.key}`] = false;
        });
    }

    private setupMouse() {
        window.addEventListener('mousedown', (e) => {
            this.mouseX = e.clientX; this.mouseY = e.clientY;
            this.isPointerDown = true;
            if (e.button === 0) this.keys.attack = true;
            if (e.button === 2) this.keys.interact = true;
        });
        window.addEventListener('mouseup', () => { this.isPointerDown = false; this.keys.attack = false; this.keys.interact = false; });
        window.addEventListener('mousemove', (e) => { this.mouseX = e.clientX; this.mouseY = e.clientY; });
        window.addEventListener('contextmenu', e => e.preventDefault());
        window.addEventListener('wheel', (e) => { this.zoomDelta = -Math.sign(e.deltaY) * 0.1; });
    }

    private setupTouch() {
        this.resizeButtons();
        window.addEventListener('resize', () => { this.resizeButtons(); });
        window.addEventListener('touchstart', (e) => this.handleTouch(e), { passive: false });
        window.addEventListener('touchmove', (e) => this.handleTouch(e), { passive: false });
        window.addEventListener('touchend', (e) => this.handleTouch(e), { passive: false });
    }

    private resizeButtons() {
        const w = window.innerWidth; const h = window.innerHeight;
        // Moved higher (was h-80, now h-120)
        this.btnA.x = w - 80; this.btnA.y = h - 120;
        this.btnB.x = w - 140; this.btnB.y = h - 100;
    }

    private handleTouch(e: TouchEvent) {
        e.preventDefault();
        let joystickFound = false; let aFound = false; let bFound = false;
        let uiPointerFound = false;

        for (let i = 0; i < e.touches.length; i++) {
            const t = e.touches[i];
            this.mouseX = t.clientX; this.mouseY = t.clientY;
            uiPointerFound = true;

            if (t.clientX > window.innerWidth / 2) {
                const distA = Math.hypot(t.clientX - this.btnA.x, t.clientY - this.btnA.y);
                if (distA < this.btnA.radius * 1.5) aFound = true;
                const distB = Math.hypot(t.clientX - this.btnB.x, t.clientY - this.btnB.y);
                if (distB < this.btnB.radius * 1.5) bFound = true;
            } else {
                if (!this.joystick.active) {
                    this.joystick.active = true;
                    this.joystick.origin = { x: t.clientX, y: t.clientY };
                    this.joystick.current = { x: t.clientX, y: t.clientY };
                } else if (this.joystick.origin) {
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
                }
                joystickFound = true;
            }
        }

        if (!joystickFound) { this.joystick.active = false; this.joystick.origin = null; }
        this.btnA.active = aFound; this.btnB.active = bFound;
        this.isPointerDown = uiPointerFound;
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
        
        // Exclude hotbar area from PC clicks (Hotbar is approx centered bottom, 7 slots * 60px)
        const hotbarW = 7 * 60;
        const hotbarX = (window.innerWidth - hotbarW) / 2;
        const hotbarY = window.innerHeight - 70;
        const isClickingHotbar = this.mouseX >= hotbarX && this.mouseX <= hotbarX + hotbarW && this.mouseY >= hotbarY;

        let attack = false;
        if ((this.keys.attack || this.btnA.active) && !isClickingHotbar && now - this.lastAttackAt >= this.attackCooldown) {
            attack = true; this.lastAttackAt = now; this.btnA.pulse = 1.0;
        }

        let interact = false;
        if ((this.keys.interact || this.btnB.active) && now - this.lastInteractAt >= this.interactCooldown) {
            interact = true; this.lastInteractAt = now; this.btnB.pulse = 1.0;
        }

        return { dx, dy, attack, interact };
    }

    updateAnimations(dt: number) {
        if (this.btnA.pulse > 0) this.btnA.pulse -= dt / 200;
        if (this.btnB.pulse > 0) this.btnB.pulse -= dt / 200;
    }

    getZoomDelta(): number { const z = this.zoomDelta; this.zoomDelta = 0; return z; }
}