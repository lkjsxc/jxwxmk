import type { PlayerState } from '../types';

export class HUDManager {
  private hpBar: HTMLElement;
  private hungerBar: HTMLElement;
  private tempBar: HTMLElement;

  constructor() {
    this.hpBar = document.getElementById('hp-bar')!;
    this.hungerBar = document.getElementById('hunger-bar')!;
    this.tempBar = document.getElementById('temp-bar')!;
  }

  update(state: PlayerState): void {
    const { vitals } = state;

    // Update HP bar
    const hpPct = (vitals.hp / vitals.max_hp) * 100;
    this.hpBar.style.width = `${Math.max(0, Math.min(100, hpPct))}%`;

    // Update hunger bar
    const hungerPct = (vitals.hunger / vitals.max_hunger) * 100;
    this.hungerBar.style.width = `${Math.max(0, Math.min(100, hungerPct))}%`;

    // Update temperature bar (show cold as inverse)
    // temperature 50 is neutral, 0 is freezing, 100 is overheating
    const tempPct = (vitals.temperature / vitals.max_temperature) * 100;
    this.tempBar.style.width = `${Math.max(0, Math.min(100, tempPct))}%`;

    // Color changes for critical values
    if (vitals.hp < vitals.max_hp * 0.25) {
      this.hpBar.style.background = '#ff0000';
    } else {
      this.hpBar.style.background = '#ff4444';
    }

    if (vitals.hunger < vitals.max_hunger * 0.25) {
      this.hungerBar.style.background = '#ff6600';
    } else {
      this.hungerBar.style.background = '#ffaa44';
    }

    if (vitals.temperature < 20 || vitals.temperature > 80) {
      this.tempBar.style.background = '#ff4444';
    } else {
      this.tempBar.style.background = '#4444ff';
    }
  }

  show(): void {
    const hud = document.getElementById('hud')!;
    hud.classList.remove('hidden');
  }

  hide(): void {
    const hud = document.getElementById('hud')!;
    hud.classList.add('hidden');
  }
}

export const hud = new HUDManager();
