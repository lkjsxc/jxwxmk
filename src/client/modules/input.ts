import { InputState } from "../types";

export class InputManager {
    keys = { 
        w: false, a: false, s: false, d: false, 
        attack: false, interact: false,
        num0: false, num1: false, num2: false, num3: false, num4: false,
        num5: false, num6: false, num7: false, num8: false, num9: false
    };
    joystick: { active: boolean; origin: { x: number; y: number } | null; current: { x: number; y: number } } = {
        active: false,
        origin: null,
        current: { x: 0, y: 0 }
    };
    
    // A Button (Attack)
    btnA = { active: false, x: 0, y: 0, radius: 35, label: 'A' };
    // B Button (Interact)
    btnB = { active: false, x: 0, y: 0, radius: 25, label: 'B' };

    zoomDelta: number = 0;
    mouseX: number = 0;
    mouseY: number = 0;
    mouseLeftDown: boolean = false;

    constructor() {
        this.setupKeyboard();
        this.setupTouch();
    }

    private setupKeyboard() {
        window.addEventListener('keydown', (e) => {
            const k = e.key.toLowerCase();
            if (k in this.keys) this.keys[k as keyof typeof this.keys] = true;
            if (e.code === 'KeyE') this.keys.interact = true;
            
            // Map digits
            if (e.key >= '0' && e.key <= '9') {
                const keyName = `num${e.key}` as keyof typeof this.keys;
                this.keys[keyName] = true;
            }
        });
        window.addEventListener('keyup', (e) => {
            const k = e.key.toLowerCase();
            if (k in this.keys) this.keys[k as keyof typeof this.keys] = false;
            if (e.code === 'KeyE') this.keys.interact = false;
            
            if (e.key >= '0' && e.key <= '9') {
                const keyName = `num${e.key}` as keyof typeof this.keys;
                this.keys[keyName] = false;
            }
        });
        window.addEventListener('mousedown', (e) => {
            if (e.button === 0) {
                this.keys.attack = true;
                this.mouseLeftDown = true;
            }
            if (e.button === 2) this.keys.interact = true;
        });
        window.addEventListener('mouseup', (e) => {
            if (e.button === 0) {
                this.keys.attack = false;
                this.mouseLeftDown = false;
            }
            if (e.button === 2) this.keys.interact = false;
        });
        window.addEventListener('mousemove', (e) => {
            this.mouseX = e.clientX;
            this.mouseY = e.clientY;
        });
        window.addEventListener('contextmenu', e => e.preventDefault());

        window.addEventListener('wheel', (e) => {
            this.zoomDelta = -Math.sign(e.deltaY) * 0.1;
        });
    }

    private setupTouch() {
        this.resizeButtons();
        window.addEventListener('resize', () => this.resizeButtons());

        window.addEventListener('touchstart', (e) => this.handleTouch(e), { passive: false });
        window.addEventListener('touchmove', (e) => this.handleTouch(e), { passive: false });
        window.addEventListener('touchend', (e) => this.handleTouch(e), { passive: false });
    }

    private resizeButtons() {
        // Bottom Right Corner
        const w = window.innerWidth;
        const h = window.innerHeight;
        
        // A (Attack) - Outer
        this.btnA.x = w - 80;
        this.btnA.y = h - 80;

        // B (Interact) - Inner/Upper
        this.btnB.x = w - 140; // Left of A
        this.btnB.y = h - 60;  // Slightly higher? Or inline? Let's put it closer to thumb arc.
        // Actually typically B is to the right/down or left/up. Let's do B to the left.
    }

    private handleTouch(e: TouchEvent) {
        e.preventDefault();
        let joystickFound = false;
        let aFound = false;
        let bFound = false;

        for (let i = 0; i < e.touches.length; i++) {
            const t = e.touches[i];
            
            // Check Buttons
            if (t.clientX > window.innerWidth / 2) {
                const distA = Math.hypot(t.clientX - this.btnA.x, t.clientY - this.btnA.y);
                if (distA < this.btnA.radius * 1.5) aFound = true;

                const distB = Math.hypot(t.clientX - this.btnB.x, t.clientY - this.btnB.y);
                if (distB < this.btnB.radius * 1.5) bFound = true;
            } 
            // Left side = Joystick
            else {
                if (!this.joystick.active) {
                    this.joystick.active = true;
                    this.joystick.origin = { x: t.clientX, y: t.clientY };
                    this.joystick.current = { x: t.clientX, y: t.clientY };
                } else if (this.joystick.origin) {
                    this.joystick.current = { x: t.clientX, y: t.clientY };
                }
                joystickFound = true;
            }
        }

        if (!joystickFound) {
            this.joystick.active = false;
            this.joystick.origin = null;
        }
        this.btnA.active = aFound;
        this.btnB.active = bFound;
    }

    getState(): InputState {
        let dx = 0;
        let dy = 0;

        // Keyboard
        if (this.keys.w) dy -= 1;
        if (this.keys.s) dy += 1;
        if (this.keys.a) dx -= 1;
        if (this.keys.d) dx += 1;

        // Joystick Override
        if (this.joystick.active && this.joystick.origin) {
            const maxRange = 50;
            let jdx = this.joystick.current.x - this.joystick.origin.x;
            let jdy = this.joystick.current.y - this.joystick.origin.y;
            
            const dist = Math.hypot(jdx, jdy);
            if (dist > 0) {
                const speed = Math.min(dist, maxRange) / maxRange;
                dx = (jdx / dist) * speed;
                dy = (jdy / dist) * speed;
            }
        } else if (dx !== 0 && dy !== 0) {
            dx *= 0.707;
            dy *= 0.707;
        }

        return {
            dx,
            dy,
            attack: this.keys.attack || this.btnA.active,
            interact: this.keys.interact || this.btnB.active
        };
    }

    getZoomDelta(): number {
        const z = this.zoomDelta;
        this.zoomDelta = 0; // Consume
        return z;
    }
}