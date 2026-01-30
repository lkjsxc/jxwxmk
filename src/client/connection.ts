import type { ServerMessage, ClientMessage } from './types';

type MessageHandler = (msg: ServerMessage) => void;
type CloseHandler = (code: number, reason: string) => void;

// Keepalive interval - must be less than server's CLIENT_TIMEOUT (10 seconds)
const KEEPALIVE_INTERVAL_MS = 5000; // Send keepalive every 5 seconds

export class ConnectionManager {
  private ws: WebSocket | null = null;
  private token: string | null = null;
  private messageHandlers: MessageHandler[] = [];
  private closeHandlers: CloseHandler[] = [];
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 3;
  private reconnectDelay = 1000;
  private keepaliveInterval: number | null = null;
  private lastActivity: number = Date.now();

  get isConnected(): boolean {
    return this.ws?.readyState === WebSocket.OPEN;
  }

  get sessionToken(): string | null {
    return this.token;
  }

  setSessionToken(token: string | null): void {
    this.token = token;
    if (token) {
      localStorage.setItem('jxwxmk_token', token);
    } else {
      localStorage.removeItem('jxwxmk_token');
    }
  }

  loadStoredToken(): string | null {
    const stored = localStorage.getItem('jxwxmk_token');
    if (stored) {
      this.token = stored;
    }
    return this.token;
  }

  clearToken(): void {
    this.token = null;
    localStorage.removeItem('jxwxmk_token');
  }

  onMessage(handler: MessageHandler): () => void {
    this.messageHandlers.push(handler);
    return () => {
      const idx = this.messageHandlers.indexOf(handler);
      if (idx >= 0) this.messageHandlers.splice(idx, 1);
    };
  }

  onClose(handler: CloseHandler): () => void {
    this.closeHandlers.push(handler);
    return () => {
      const idx = this.closeHandlers.indexOf(handler);
      if (idx >= 0) this.closeHandlers.splice(idx, 1);
    };
  }

  connect(): void {
    if (this.ws?.readyState === WebSocket.CONNECTING) return;
    if (this.ws?.readyState === WebSocket.OPEN) return;

    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const host = window.location.host;
    const tokenParam = this.token ? `?token=${encodeURIComponent(this.token)}` : '';
    const url = `${protocol}//${host}/ws${tokenParam}`;

    console.log('[Connection] Connecting to:', url);
    this.ws = new WebSocket(url);

    this.ws.onopen = () => {
      console.log('[Connection] WebSocket connected');
      this.reconnectAttempts = 0;
      this.lastActivity = Date.now();
      this.startKeepalive();
    };

    this.ws.onmessage = (event) => {
      this.lastActivity = Date.now();
      try {
        const msg: ServerMessage = JSON.parse(event.data);
        this.handleMessage(msg);
      } catch (err) {
        console.error('[Connection] Failed to parse message:', err);
      }
    };

    this.ws.onclose = (event) => {
      console.log('[Connection] WebSocket closed:', event.code, event.reason);
      this.stopKeepalive();
      this.ws = null;
      this.closeHandlers.forEach(h => h(event.code, event.reason));
    };

    this.ws.onerror = (err) => {
      console.error('[Connection] WebSocket error:', err);
    };
  }

  disconnect(): void {
    this.stopKeepalive();
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  send(msg: ClientMessage): boolean {
    if (!this.isConnected) return false;
    try {
      this.ws!.send(JSON.stringify(msg));
      this.lastActivity = Date.now();
      return true;
    } catch (err) {
      console.error('[Connection] Failed to send message:', err);
      return false;
    }
  }

  private startKeepalive(): void {
    this.stopKeepalive();
    this.keepaliveInterval = window.setInterval(() => {
      if (!this.isConnected) {
        this.stopKeepalive();
        return;
      }

      // Send a minimal input message as keepalive if no other activity
      const timeSinceActivity = Date.now() - this.lastActivity;
      if (timeSinceActivity > KEEPALIVE_INTERVAL_MS - 1000) {
        // Send empty input to keep connection alive
        this.send({
          type: 'input',
          data: { dx: 0, dy: 0, attack: false, interact: false }
        });
      }
    }, KEEPALIVE_INTERVAL_MS);
  }

  private stopKeepalive(): void {
    if (this.keepaliveInterval !== null) {
      clearInterval(this.keepaliveInterval);
      this.keepaliveInterval = null;
    }
  }

  private handleMessage(msg: ServerMessage): void {
    // Handle welcome message to store token
    if (msg.type === 'welcome') {
      this.setSessionToken(msg.token);
    }

    // Handle session revoked
    if (msg.type === 'sessionRevoked') {
      this.clearToken();
    }

    // Notify all handlers
    this.messageHandlers.forEach(h => h(msg));
  }
}

export const connection = new ConnectionManager();
