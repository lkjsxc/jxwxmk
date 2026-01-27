export interface QuestObjective {
  kind: string;
  count: number;
  current: number;
}

export interface QuestEntry {
  id: string;
  name: string;
  state: string;
  objectives: QuestObjective[];
}

export class QuestState {
  quests: QuestEntry[] = [];
  pinnedQuestId: string | null = null;

  applyUpdate(entry: QuestEntry): void {
    const idx = this.quests.findIndex((quest) => quest.id === entry.id);
    if (idx >= 0) {
      this.quests[idx] = entry;
    } else {
      this.quests.push(entry);
    }
  }

  pin(id: string): void {
    this.pinnedQuestId = this.pinnedQuestId === id ? null : id;
  }

  getPinned(): QuestEntry | null {
    if (!this.pinnedQuestId) {
      return null;
    }
    return this.quests.find((quest) => quest.id === this.pinnedQuestId) || null;
  }
}
