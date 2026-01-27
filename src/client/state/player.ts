export interface PlayerProfile {
  id: string | null;
  token: string | null;
  name: string;
  level: number;
  xp: number;
  stats: { kills: number; deaths: number; crafts: number; steps: number };
  sessionState: "connected" | "revoked" | "invalid";
}

const TOKEN_KEY = "jxwxmk_token";

export function loadToken(): string | null {
  return localStorage.getItem(TOKEN_KEY);
}

export function saveToken(token: string): void {
  localStorage.setItem(TOKEN_KEY, token);
}

export function clearToken(): void {
  localStorage.removeItem(TOKEN_KEY);
}

export function createDefaultProfile(): PlayerProfile {
  return {
    id: null,
    token: loadToken(),
    name: "Traveler",
    level: 1,
    xp: 0,
    stats: { kills: 0, deaths: 0, crafts: 0, steps: 0 },
    sessionState: "connected",
  };
}
