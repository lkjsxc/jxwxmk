import { InputState } from "../types";

export class InputManager {
    keys = { w: false, a: false, s: false, d: false, attack: false };
    joystick: { active: boolean; origin: { x: number; y: number } | null; current: { x: number; y: number } } = {
        active: false,
        origin: null,
        current: { x: 0, y: 0 }
    };
    attackBtn = { active: false, x: 0, y: 0, radius: 40 };

    constructor() {
        this.setupKeyboard();
        this.setupTouch();
    }

    private setupKeyboard() {
        window.addEventListener('keydown', (e) => {
            const k = e.key.toLowerCase();
            if (k in this.keys) this.keys[k as keyof typeof this.keys] = true;
        });
        window.addEventListener('keyup', (e) => {
            const k = e.key.toLowerCase();
            if (k in this.keys) this.keys[k as keyof typeof this.keys] = false;
        });
        window.addEventListener('mousedown', () => { this.keys.attack = true; });
        window.addEventListener('mouseup', () => { this.keys.attack = false; });
    }

    private setupTouch() {
        // Attack Button Position (Bottom Right)
        this.attackBtn.x = window.innerWidth - 80;
        this.attackBtn.y = window.innerHeight - 80;

        window.addEventListener('resize', () => {
            this.attackBtn.x = window.innerWidth - 80;
            this.attackBtn.y = window.innerHeight - 80;
        });

        window.addEventListener('touchstart', (e) => this.handleTouch(e), { passive: false });
        window.addEventListener('touchmove', (e) => this.handleTouch(e), { passive: false });
        window.addEventListener('touchend', (e) => this.handleTouch(e), { passive: false });
    }

    private handleTouch(e: TouchEvent) {
        e.preventDefault();
        let joystickFound = false;
        let attackFound = false;

        for (let i = 0; i < e.touches.length; i++) {
            const t = e.touches[i];
            
            // Right side = Attack
            if (t.clientX > window.innerWidth / 2) {
                const dist = Math.hypot(t.clientX - this.attackBtn.x, t.clientY - this.attackBtn.y);
                if (dist < this.attackBtn.radius * 2) {
                    attackFound = true;
                }
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
        this.attackBtn.active = attackFound;
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
            
            // Normalize
            const dist = Math.hypot(jdx, jdy);
            if (dist > 0) {
                const speed = Math.min(dist, maxRange) / maxRange;
                dx = (jdx / dist) * speed;
                dy = (jdy / dist) * speed;
            }
        } else if (dx !== 0 && dy !== 0) {
             // Keyboard Normalize
            dx *= 0.707;
            dy *= 0.707;
        }

        return {
            dx,
            dy,
            attack: this.keys.attack || this.attackBtn.active
        };
    }
}
