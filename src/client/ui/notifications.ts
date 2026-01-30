import type { Notification } from '../types';

const NOTIFICATION_DURATION_MS = 3000;

export class NotificationManager {
  private notifications: Notification[] = [];
  private nextId = 1;
  private container: HTMLElement;

  constructor() {
    this.container = document.getElementById('notifications')!;
  }

  show(text: string, type: Notification['type'] = 'info'): void {
    const notification: Notification = {
      id: this.nextId++,
      text,
      type,
      createdAt: Date.now(),
    };

    this.notifications.push(notification);
    this.render();

    // Auto-remove after duration
    setTimeout(() => {
      this.remove(notification.id);
    }, NOTIFICATION_DURATION_MS);
  }

  remove(id: number): void {
    const idx = this.notifications.findIndex(n => n.id === id);
    if (idx >= 0) {
      this.notifications.splice(idx, 1);
      this.render();
    }
  }

  private render(): void {
    this.container.innerHTML = '';

    // Show only the most recent notification
    const notification = this.notifications[this.notifications.length - 1];
    if (!notification) return;

    const el = document.createElement('div');
    el.className = `notification notification-${notification.type}`;
    el.textContent = notification.text;

    // Add type-specific styling
    if (notification.type === 'error') {
      el.style.borderLeftColor = '#ff4444';
    } else if (notification.type === 'achievement') {
      el.style.borderLeftColor = '#ffd700';
    }

    this.container.appendChild(el);
  }

  clear(): void {
    this.notifications = [];
    this.render();
  }
}

export const notifications = new NotificationManager();
