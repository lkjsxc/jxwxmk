import { EventEmitter } from 'events';
import { InputState } from '../input/InputState';

export class NetworkManager extends EventEmitter {
    private socket: WebSocket | null;
    private serverUrl: string;
    private reconnectAttempts: number;
    private maxReconnectAttempts: number;
    private reconnectDelay: number;
    private authenticated: boolean;
    private playerId: string | null;
    private messageSequence: number;
    
    constructor(serverUrl: string) {
        super();
        this.serverUrl = serverUrl;
        this.socket = null;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 1000;
        this.authenticated = false;
        this.playerId = null;
        this.messageSequence = 0;
    }
    
    public async connect(): Promise<void> {
        return new Promise((resolve, reject) => {
            try {
                console.log(`Connecting to server: ${this.serverUrl}`);
                
                this.socket = new WebSocket(this.serverUrl);
                
                this.socket.onopen = () => {
                    console.log('WebSocket connection established');
                    this.reconnectAttempts = 0;
                    this.authenticate().then(resolve).catch(reject);
                };
                
                this.socket.onmessage = (event) => {
                    this.handleMessage(event.data);
                };
                
                this.socket.onclose = (event) => {
                    console.log('WebSocket connection closed:', event.code, event.reason);
                    this.handleDisconnect();
                };
                
                this.socket.onerror = (error) => {
                    console.error('WebSocket error:', error);
                    this.handleDisconnect();
                };
                
            } catch (error) {
                console.error('Connection error:', error);
                reject(error);
            }
        });
    }
    
    private async authenticate(): Promise<void> {
        // In a real implementation, this would use proper authentication
        // For now, we'll use a simple token
        const authMessage = {
            type: 'authenticate',
            token: 'dev_token',
            protocolVersion: 1
        };
        
        this.sendMessage(authMessage);
    }
    
    public disconnect(): void {
        if (this.socket) {
            this.socket.close(1000, 'Client disconnect');
            this.socket = null;
        }
        this.authenticated = false;
        this.playerId = null;
    }
    
    private handleDisconnect(): void {
        this.authenticated = false;
        this.playerId = null;
        this.emit('disconnected');
        
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            console.log(`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})...`);
            
            setTimeout(() => {
                this.connect().catch(error => {
                    console.error('Reconnection failed:', error);
                });
            }, this.reconnectDelay);
        } else {
            console.log('Max reconnection attempts reached');
        }
    }
    
    private handleMessage(data: any): void {
        try {
            if (typeof data === 'string') {
                const message = JSON.parse(data);
                this.processMessage(message);
            } else if (data instanceof ArrayBuffer) {
                // Handle binary messages
                const view = new DataView(data);
                const protocolVersion = view.getUint8(0);
                const messageType = view.getUint8(1);
                const sequence = view.getUint32(2, true);
                
                // Parse payload based on message type
                // This would be implemented based on the binary protocol
                console.log(`Binary message received: type=${messageType}, seq=${sequence}`);
            }
        } catch (error) {
            console.error('Error handling message:', error);
            this.emit('error', 'Message processing error');
        }
    }
    
    private processMessage(message: any): void {
        switch (message.type) {
            case 'authenticated':
                this.authenticated = true;
                this.playerId = message.playerId;
                this.emit('authenticated', message.playerId);
                break;
                
            case 'stateUpdate':
                this.emit('stateUpdate', message);
                break;
                
            case 'error':
                this.emit('error', message.message);
                break;
                
            case 'pong':
                this.emit('pong', message);
                break;
                
            default:
                console.warn('Unknown message type:', message.type);
        }
    }
    
    public sendPlayerInput(input: InputState): void {
        if (!this.authenticated || !this.socket) {
            return;
        }
        
        const message = {
            type: 'input',
            input: {
                movement: input.movement,
                actions: input.actions,
                sprint: input.sprint
            },
            sequence: this.messageSequence++
        };
        
        this.sendMessage(message);
    }
    
    public sendPing(): void {
        if (!this.socket) return;
        
        const message = {
            type: 'ping'
        };
        
        this.sendMessage(message);
    }
    
    private sendMessage(message: any): void {
        if (!this.socket || this.socket.readyState !== WebSocket.OPEN) {
            console.warn('Cannot send message - socket not open');
            return;
        }
        
        try {
            const json = JSON.stringify(message);
            this.socket.send(json);
        } catch (error) {
            console.error('Failed to send message:', error);
        }
    }
    
    public isConnected(): boolean {
        return this.socket !== null && this.socket.readyState === WebSocket.OPEN;
    }
    
    public isAuthenticated(): boolean {
        return this.authenticated;
    }
    
    public getPlayerId(): string | null {
        return this.playerId;
    }
}