import { QuestState } from "./types";

export class PlayerSession {
  id: string | null = null;
  token: string | null = null;
  spawned = false;
  sessionRevoked = false;
  quests: QuestState[] = [];
  pinnedQuestId: string | null = null;
  pinnedAchievementId: string | null = null;
  lastSeenAt = Date.now();
  achievements = new Set<string>();

  setWelcome(id: string, token: string, spawned: boolean) {
    this.id = id;
    this.token = token;
    this.spawned = spawned;
    this.sessionRevoked = false;
    this.lastSeenAt = Date.now();
  }
}
