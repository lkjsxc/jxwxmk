export class InputHandler {
    private keys = new Set<string>();

    constructor(private canvas: HTMLCanvasElement, private client: any) {
        window.addEventListener('keydown', (e) => this.keys.add(e.key));
        window.addEventListener('keyup', (e) => this.keys.delete(e.key));
    }

    update() {
        if (this.keys.has('ArrowUp')) {
            this.client.sendInput('move', new Uint8Array([0, 1]));
        }
        // etc.
    }
}