export class GameClient {
    private ws: WebSocket | null = null;
    private seq = 0;

    constructor(private url: string) {}

    connect() {
        this.ws = new WebSocket(this.url);
        this.ws.binaryType = 'arraybuffer';
        this.ws.onopen = () => console.log('Connected');
        this.ws.onmessage = (e) => this.handleMessage(e);
        this.ws.onclose = () => console.log('Disconnected');
    }

    private handleMessage(event: MessageEvent) {
        if (event.data instanceof ArrayBuffer) {
            const view = new DataView(event.data);
            const version = view.getUint32(0, true);
            if (version !== 1) return;
            // Parse and update game state
            console.log('Received snapshot');
        }
    }

    sendInput(action: string, data: Uint8Array) {
        if (!this.ws) return;
        const message = {
            protocol_version: 1,
            msg_type: { Input: { player_id: 123, action, data: Array.from(data) } },
            seq: this.seq++,
            payload: [],
        };
        const bytes = this.serialize(message);
        this.ws.send(bytes);
    }

    private serialize(msg: any): Uint8Array {
        // Placeholder: manual bincode-like
        const buffer = new ArrayBuffer(1024);
        const view = new DataView(buffer);
        view.setUint32(0, msg.protocol_version, true);
        // ... more
        return new Uint8Array(buffer.slice(0, 100));  // Truncated
    }
}