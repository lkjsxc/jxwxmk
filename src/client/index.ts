interface Vec2 { x: number; y: number; }

interface Entity {
    id: string;
    kind: string;
    subtype: string;
    x: number;
    y: number;
}

class GameClient {
    private socket: WebSocket | null = null;
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;
    private entities: Map<string, Entity> = new Map();
    private playerId: string | null = null;

    constructor() {
        this.canvas = document.getElementById('game-canvas') as HTMLCanvasElement;
        this.ctx = this.canvas.getContext('2d')!;
        this.resize();
        window.addEventListener('resize', () => this.resize());
        this.setupInput();
    }

    private resize() {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
    }

    private setupInput() {
        window.addEventListener('keydown', (e) => {
            let dx = 0, dy = 0;
            if (e.key === 'w') dy = -1;
            if (e.key === 's') dy = 1;
            if (e.key === 'a') dx = -1;
            if (e.key === 'd') dx = 1;
            if (dx !== 0 || dy !== 0) {
                this.sendInput(dx, dy);
            }
        });
    }

    private sendInput(dx: number, dy: number) {
        if (this.socket?.readyState === WebSocket.OPEN) {
            this.socket.send(JSON.stringify({
                type: 'input',
                data: { dx, dy, attack: false, interact: false, aim: null }
            }));
        }
    }

    public connect() {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        this.socket = new WebSocket(`${protocol}//${window.location.host}/ws`);
        
        this.socket.onmessage = (event) => {
            const msg = JSON.parse(event.data);
            this.handleMessage(msg);
        };
    }

    private handleMessage(msg: any) {
        switch (msg.type) {
            case 'welcome':
                this.playerId = msg.id;
                break;
            case 'entityDelta':
                for (const update of msg.data.updates) {
                    this.entities.set(update.id, update);
                }
                for (const remove of msg.data.removes) {
                    this.entities.delete(remove.id);
                }
                break;
        }
    }

    public startLoop() {
        const loop = () => {
            this.render();
            requestAnimationFrame(loop);
        };
        requestAnimationFrame(loop);
    }

    private render() {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        this.ctx.fillStyle = '#222';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        for (const entity of this.entities.values()) {
            this.ctx.fillStyle = entity.id === this.playerId ? '#0f0' : '#f00';
            this.ctx.beginPath();
            this.ctx.arc(
                this.canvas.width / 2 + (entity.x * 20),
                this.canvas.height / 2 + (entity.y * 20),
                10, 0, Math.PI * 2
            );
            this.ctx.fill();
        }
    }
}

const client = new GameClient();
client.connect();
client.startLoop();