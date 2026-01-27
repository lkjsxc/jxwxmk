use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::item::{ItemId, ItemStack};

pub type PlayerId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub steps: u64,
    pub kills: u64,
    pub crafts: u64,
    pub gathers: u64,
    pub deaths: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatBonus {
    pub stat: String,
    pub value: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            steps: 0,
            kills: 0,
            crafts: 0,
            gathers: 0,
            deaths: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySlot {
    pub item: Option<ItemStack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub slots: Vec<InventorySlot>,
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![InventorySlot { item: None }; size],
        }
    }

    pub fn get(&self, index: usize) -> Option<&InventorySlot> {
        self.slots.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut InventorySlot> {
        self.slots.get_mut(index)
    }

    pub fn add_item(&mut self, item: ItemId, count: u32) -> bool {
        if let Some(slot) = self
            .slots
            .iter_mut()
            .find(|slot| slot.item.as_ref().map(|s| s.item == item).unwrap_or(false))
        {
            if let Some(stack) = slot.item.as_mut() {
                stack.count = stack.count.saturating_add(count);
                return true;
            }
        }

        if let Some(slot) = self.slots.iter_mut().find(|slot| slot.item.is_none()) {
            slot.item = Some(ItemStack::new(item, count));
            return true;
        }

        false
    }

    pub fn remove_item(&mut self, item: &ItemId, count: u32) -> bool {
        if let Some(slot) = self
            .slots
            .iter_mut()
            .find(|slot| slot.item.as_ref().map(|s| &s.item == item).unwrap_or(false))
        {
            if let Some(stack) = slot.item.as_mut() {
                if stack.count < count {
                    return false;
                }
                stack.count -= count;
                if stack.count == 0 {
                    slot.item = None;
                }
                return true;
            }
        }
        false
    }

    pub fn swap(&mut self, from: usize, to: usize) -> bool {
        if from >= self.slots.len() || to >= self.slots.len() {
            return false;
        }
        self.slots.swap(from, to);
        true
    }

    pub fn item_count(&self, item: &ItemId) -> u32 {
        self.slots
            .iter()
            .filter_map(|slot| slot.item.as_ref())
            .filter(|stack| &stack.item == item)
            .map(|stack| stack.count)
            .sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub id: PlayerId,
    pub token: Uuid,
    pub username: String,
    pub x: f32,
    pub y: f32,
    pub chunk: (i32, i32),
    pub health: f32,
    pub hunger: f32,
    pub temperature: f32,
    pub thirst: f32,
    pub inventory: Inventory,
    pub active_slot: usize,
    pub stats: PlayerStats,
    pub stat_bonuses: Vec<StatBonus>,
    pub level: u32,
    pub xp: i64,
    pub achievements: Vec<String>,
    pub quests: Vec<PlayerQuest>,
    pub reputation: Vec<ReputationEntry>,
    pub spawned: bool,
    pub last_attack_tick: u64,
    pub last_interact_tick: u64,
}

impl PlayerState {
    pub fn new(id: PlayerId, token: Uuid, inventory_slots: usize, max_health: f32) -> Self {
        Self {
            id,
            token,
            username: "Traveler".to_string(),
            x: 0.0,
            y: 0.0,
            chunk: (0, 0),
            health: max_health,
            hunger: 100.0,
            temperature: 50.0,
            thirst: 100.0,
            inventory: Inventory::new(inventory_slots),
            active_slot: 0,
            stats: PlayerStats::default(),
            stat_bonuses: Vec::new(),
            level: 1,
            xp: 0,
            achievements: Vec::new(),
            quests: Vec::new(),
            reputation: Vec::new(),
            spawned: false,
            last_attack_tick: 0,
            last_interact_tick: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerQuest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub state: String,
    pub objectives: Vec<PlayerQuestObjective>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerQuestObjective {
    pub kind: String,
    pub target: Option<String>,
    pub count: u32,
    pub current: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationEntry {
    pub key: String,
    pub tier: u32,
}
