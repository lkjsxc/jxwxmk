export class WebSocketService {
    private ws: WebSocket | null = null;
    private url: string;
    private reconnectAttempts: number = 0;
    private maxReconnectAttempts: number = 5;

    public onMessage: (data: any) => void;
    public onConnect: () => void;
    public onError: (error: Event) => void;

    constructor(url: string) {
        this.url = url;
    }

    public connect(): void {
        try {
            this.ws = new WebSocket(this.url);
            this.setupEventListeners();
        } catch (error) {
            console.error('Failed to connect:', error);
            this.attemptReconnect();
        }
    }

    public send(data: any): void {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(data));
        }
    }

    public sendGameState(gameState: any): void {
        this.send({
            type: 'game_state_update',
            payload: gameState
        });
    }

    public disconnect(): void {
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
    }

    private setupEventListeners(): void {
        if (!this.ws) return;

        this.ws.onopen = () => {
            console.log('WebSocket connected');
            this.reconnectAttempts = 0;
            if (this.onConnect) {
                this.onConnect();
            }
        };

        this.ws.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);
                if (this.onMessage) {
                    this.onMessage(data);
                }
            } catch (error) {
                console.error('Failed to parse message:', error);
            }
        };

        this.ws.onclose = (event) => {
            console.log('WebSocket disconnected:', event.code, event.reason);
            this.attemptReconnect();
        };

        this.ws.onerror = (error) => {
            console.error('WebSocket error:', error);
            if (this.onError) {
                this.onError(error);
            }
        };
    }

    private attemptReconnect(): void {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            console.log(`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})...`);
            
            setTimeout(() => {
                this.connect();
            }, 1000 * this.reconnectAttempts);
        } else {
            console.error('Max reconnection attempts reached');
        }
    }
}