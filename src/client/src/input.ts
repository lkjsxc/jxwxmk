export class InputManager {
    public keys: Set<string> = new HashSet();
    public mouseX: number = 0;
    public mouseY: number = 0;
    public attack: boolean = false;
    public interact: boolean = false;

    constructor() {
        window.addEventListener('keydown', (e) => this.keys.add(e.code));
        window.addEventListener('keyup', (e) => this.keys.delete(e.code));
        window.addEventListener('mousemove', (e) => {
            this.mouseX = e.clientX;
            this.mouseY = e.clientY;
        });
        window.addEventListener('mousedown', (e) => {
            if (e.button === 0) this.attack = true;
            if (e.button === 2) this.interact = true;
        });
        window.addEventListener('mouseup', (e) => {
            if (e.button === 0) this.attack = false;
            if (e.button === 2) this.interact = false;
        });
        window.addEventListener('contextmenu', (e) => e.preventDefault());
    }

    getMovement(): { dx: number, dy: number } {
        let dx = 0;
        let dy = 0;
        if (this.keys.has('KeyW')) dy -= 1;
        if (this.keys.has('KeyS')) dy += 1;
        if (this.keys.has('KeyA')) dx -= 1;
        if (this.keys.has('KeyD')) dx += 1;
        return { dx, dy };
    }
}

class HashSet<T> extends Set<T> {} // Simple polyfill/wrapper
