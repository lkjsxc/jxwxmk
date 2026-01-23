export class GameClient {
    private socket: WebSocket | null = null;
    private gameState: any = {};

    connect() {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const host = window.location.host;
        const socketUrl = `${protocol}//${host}/ws`;

        this.socket = new WebSocket(socketUrl);

        this.socket.onopen = () => {
            console.log('Connected to game server');
        };

        this.socket.onmessage = (event) => {
            this.gameState = JSON.parse(event.data);
        };

        this.socket.onclose = () => {
            console.log('Disconnected from game server');
        };
    }

    getState() {
        return this.gameState;
    }
}