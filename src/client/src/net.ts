import { ClientMessage, ServerMessage } from './protocol';

export type MessageHandler = (msg: ServerMessage) => void;

export class NetManager {
    private ws: WebSocket | null = null;
    private handlers: MessageHandler[] = [];

    constructor(private onOpen: () => void, private onClose: () => void) {}

    connect(token: string) {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const host = window.location.host;
        this.ws = new WebSocket(`${protocol}//${host}/ws?token=${token}`);

        this.ws.onopen = () => {
            console.log('Connected to server');
            this.onOpen();
        };

        this.ws.onclose = () => {
            console.log('Disconnected from server');
            this.onClose();
        };

        this.ws.onmessage = (event) => {
            try {
                const msg = JSON.parse(event.data) as ServerMessage;
                for (const handler of this.handlers) {
                    handler(msg);
                }
            } catch (e) {
                console.error('Failed to parse message', e, event.data);
            }
        };
    }

    send(msg: ClientMessage) {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(msg));
        }
    }

    addHandler(handler: MessageHandler) {
        this.handlers.push(handler);
    }
}
