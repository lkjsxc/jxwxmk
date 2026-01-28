export type LoginCallbacks = {
  onClaimSession: () => void;
  onConnect: () => void;
};

export class LoginManager {
  private loginScreen: HTMLElement | null;
  private sessionInfo: HTMLElement | null;
  private tokenEl: HTMLElement | null;

  constructor(private callbacks: LoginCallbacks) {
    this.loginScreen = document.getElementById('login-screen');
    this.sessionInfo = document.getElementById('session-info');
    this.tokenEl = document.getElementById('token');

    this.setupListeners();
    this.loadSession();
  }

  private setupListeners(): void {
    document.getElementById('claim-session')?.addEventListener('click', () => {
      this.callbacks.onClaimSession();
    });

    document.getElementById('connect')?.addEventListener('click', () => {
      this.callbacks.onConnect();
    });
  }

  private loadSession(): void {
    const token = localStorage.getItem('sessionToken');
    const playerId = localStorage.getItem('playerId');

    if (token && playerId) {
      this.sessionInfo?.classList.remove('hidden');
      if (this.tokenEl) {
        this.tokenEl.textContent = token.substring(0, 8) + '...';
      }
    }
  }

  showSessionInfo(token: string): void {
    this.sessionInfo?.classList.remove('hidden');
    if (this.tokenEl) {
      this.tokenEl.textContent = token.substring(0, 8) + '...';
    }
  }

  hide(): void {
    this.loginScreen?.classList.add('hidden');
  }

  show(): void {
    this.loginScreen?.classList.remove('hidden');
  }
}
