import { NetManager } from './net';
import { ServerMessage, EntitySnapshot, ChunkAddData, InventorySlot } from './protocol';

export class Game {
    public playerID: string | null = null;
    public inventory: (InventorySlot | null)[] = [];
    public activeQuests: any[] = [];
    public unlockedAchievements: Set<string> = new Set();
    public notifications: { text: string, time: number }[] = [];
    public chunks: Map<string, ChunkAddData> = new Map();
    public entities: Map<string, EntitySnapshot> = new Map();
    public spawned: boolean = false;

    constructor(private net: NetManager) {
        this.net.addHandler(this.onMessage.bind(this));
    }

    private onMessage(msg: ServerMessage) {
        switch (msg.type) {
            case 'welcome':
                this.playerID = msg.id;
                this.spawned = msg.spawned;
                if (!this.spawned) {
                    this.net.send({ type: 'spawn', data: { settlement_id: null } });
                }
                break;
            case 'playerUpdate':
                this.inventory = msg.data.inventory;
                this.spawned = true;
                // Update local quest list and achievements
                this.activeQuests = msg.data.quests;
                break;
            case 'notification':
                this.notifications.push({ text: msg.data.text, time: Date.now() });
                break;
            case 'achievement':
                this.notifications.push({ text: `Unlocked: ${msg.data.name}`, time: Date.now() });
                this.unlockedAchievements.add(msg.data.id);
                break;
            case 'questUpdate':
                // Merge or replace quest
                const idx = this.activeQuests.findIndex(q => q.id === msg.data.quest.id);
                if (idx !== -1) {
                    this.activeQuests[idx] = msg.data.quest;
                } else {
                    this.activeQuests.push(msg.data.quest);
                }
                break;
            case 'chunkAdd':
                this.chunks.set(this.getChunkKey(msg.data.coord), msg.data);
                this.addEntitiesFromChunk(msg.data);
                break;
            case 'chunkRemove':
                this.chunks.delete(this.getChunkKey(msg.data.coord));
                // In a full implementation, we'd also remove entities tied to this chunk
                break;
            case 'entityDelta':
                for (const update of msg.data.updates) {
                    this.entities.set(update.id, update);
                }
                for (const remove of msg.data.removes) {
                    this.entities.delete(remove.id);
                }
                break;
            case 'sessionRevoked':
                alert('Session revoked: ' + msg.reason);
                window.location.reload();
                break;
            case 'error':
                console.error('Server error', msg.data);
                if (msg.data.code === 'invalid_token') {
                    localStorage.removeItem('jxwxmk_token');
                    window.location.reload();
                }
                break;
        }
    }

    private getChunkKey(coord: [number, number]): string {
        return `${coord[0]},${coord[1]}`;
    }

    private addEntitiesFromChunk(chunk: ChunkAddData) {
        const e = chunk.entities;
        const all = [e.resources, e.mobs, e.structures, e.npcs];
        for (const map of all) {
            for (const ent of Object.values(map)) {
                this.entities.set(ent.id, ent);
            }
        }
    }

    update() {
        // Local logic (interpolation etc.)
    }
}
