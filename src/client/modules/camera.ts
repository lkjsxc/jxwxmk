export class Camera {
    x: number = 0;
    y: number = 0;
    zoom: number = 1.0;
    width: number;
    height: number;
    targetX: number = 0;
    targetY: number = 0;

    constructor() {
        this.width = window.innerWidth;
        this.height = window.innerHeight;
        window.addEventListener('resize', () => this.resize());
    }

    resize() {
        this.width = window.innerWidth;
        this.height = window.innerHeight;
    }

    follow(targetX: number, targetY: number) {
        this.targetX = targetX;
        this.targetY = targetY;
    }

    update() {
        // Smooth lerp
        this.x += (this.targetX - this.x) * 0.1;
        this.y += (this.targetY - this.y) * 0.1;
    }

    setZoom(delta: number) {
        const newZoom = this.zoom + delta;
        this.zoom = Math.max(0.5, Math.min(newZoom, 2.0));
    }
}
