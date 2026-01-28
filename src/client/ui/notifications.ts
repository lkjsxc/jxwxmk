export class NotificationManager {
  private container: HTMLElement | null;

  constructor() {
    this.container = document.getElementById('notifications');
  }

  show(text: string, duration = 3000): void {
    if (!this.container) return;

    const notif = document.createElement('div');
    notif.className = 'notification';
    notif.textContent = text;

    this.container.appendChild(notif);

    setTimeout(() => {
      notif.style.opacity = '0';
      notif.style.transform = 'translateX(100%)';
      setTimeout(() => notif.remove(), 300);
    }, duration);
  }
}
