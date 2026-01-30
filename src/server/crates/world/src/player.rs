use protocol::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct PlayerState {
    pub id: Uuid,
    pub name: String,
    pub spawned: bool,
    pub x: f64,
    pub y: f64,
    pub vitals: Vitals,
    pub inventory: Vec<Option<InventorySlot>>,
    pub active_slot: usize,
    pub level: i32,
    pub xp: i64,
    pub stats: PlayerStats,
    pub quests: Vec<QuestInfo>,
    pub achievements: Vec<String>,
    pub settlement_id: Option<Uuid>,
    pub respawn_cooldown: f64,
}

impl PlayerState {
    pub fn new(id: Uuid, name: String) -> Self {
        let inventory: Vec<Option<InventorySlot>> = (0..30).map(|_| None).collect();
        
        Self {
            id,
            name,
            spawned: false,
            x: 0.0,
            y: 0.0,
            vitals: Vitals {
                hp: 100.0,
                max_hp: 100.0,
                hunger: 100.0,
                max_hunger: 100.0,
                temperature: 50.0,
                max_temperature: 100.0,
            },
            inventory,
            active_slot: 0,
            level: 1,
            xp: 0,
            stats: PlayerStats::default(),
            quests: Vec::new(),
            achievements: Vec::new(),
            settlement_id: None,
            respawn_cooldown: 0.0,
        }
    }

    pub fn spawn(&mut self, x: f64, y: f64, settlement_id: Option<Uuid>) {
        self.spawned = true;
        self.x = x;
        self.y = y;
        self.settlement_id = settlement_id;
        let mut vitals = self.vitals;
        vitals.hp = vitals.max_hp;
        vitals.hunger = vitals.max_hunger;
        vitals.temperature = 50.0;
        self.vitals = vitals;
    }

    pub fn unspawn(&mut self) {
        self.spawned = false;
    }

    pub fn is_alive(&self) -> bool {
        self.vitals.hp > 0.0
    }

    pub fn move_by(&mut self, dx: f64, dy: f64, speed: f64, dt: f64) {
        if !self.spawned {
            return;
        }
        let move_dist = speed * dt;
        self.x += dx * move_dist;
        self.y += dy * move_dist;
        self.stats.steps += 1;
    }

    pub fn take_damage(&mut self, damage: f64) {
        self.vitals.hp = (self.vitals.hp - damage).max(0.0);
    }

    pub fn heal(&mut self, amount: f64) {
        self.vitals.hp = (self.vitals.hp + amount).min(self.vitals.max_hp);
    }

    pub fn add_inventory_item(&mut self, item: String, count: i32) -> bool {
        // Try to stack first
        for slot in self.inventory.iter_mut().flatten() {
            if slot.item == item {
                slot.count += count;
                return true;
            }
        }
        
        // Find empty slot
        for slot in &mut self.inventory {
            if slot.is_none() {
                *slot = Some(InventorySlot { item, count });
                return true;
            }
        }
        
        false
    }

    pub fn remove_inventory_item(&mut self, item: &str, count: i32) -> bool {
        for slot in &mut self.inventory {
            if let Some(ref mut s) = slot {
                if s.item == item && s.count >= count {
                    s.count -= count;
                    if s.count == 0 {
                        *slot = None;
                    }
                    return true;
                }
            }
        }
        false
    }

    pub fn has_item(&self, item: &str, count: i32) -> bool {
        self.inventory.iter().flatten().any(|s| s.item == item && s.count >= count)
    }

    pub fn count_item(&self, item: &str) -> i32 {
        self.inventory
            .iter()
            .flatten()
            .filter(|s| s.item == item)
            .map(|s| s.count)
            .sum()
    }

    pub fn to_update_data(&self) -> PlayerUpdateData {
        PlayerUpdateData {
            id: self.id,
            name: self.name.clone(),
            spawned: self.spawned,
            x: self.x,
            y: self.y,
            vitals: self.vitals.clone(),
            inventory: self.inventory.clone(),
            active_slot: self.active_slot,
            level: self.level,
            xp: self.xp,
            stats: self.stats.clone(),
            quests: self.quests.clone(),
            achievements: self.achievements.clone(),
        }
    }
}
