use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::game::entities::item::{Item, ItemType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub slots: Vec<Option<Item>>,
    pub capacity: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: vec![None; 30], // 30 slots (Grid + Hotbar)
            capacity: 30,
        }
    }
}

impl Inventory {
    pub fn add(&mut self, kind: ItemType, mut amount: u32) -> u32 {
        // 1. Try to stack
        for slot in self.slots.iter_mut() {
            if let Some(item) = slot {
                if item.kind == kind && item.amount < item.max_stack {
                    let space = item.max_stack - item.amount;
                    let to_add = amount.min(space);
                    item.amount += to_add;
                    amount -= to_add;
                    if amount == 0 {
                        return 0;
                    }
                }
            }
        }

        // 2. Try to fill empty slots
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                // Determine max stack for new item
                let max_stack = Item::new(kind.clone(), 1).max_stack; 
                let to_add = amount.min(max_stack);
                *slot = Some(Item {
                    kind: kind.clone(),
                    amount: to_add,
                    max_stack,
                });
                amount -= to_add;
                if amount == 0 {
                    return 0;
                }
            }
        }
        
        amount // Return remaining
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub x: f64,
    pub y: f64,
    pub health: f64,
    pub hunger: f64,
    pub cold: f64,
    pub inventory: Inventory,
    pub active_slot: usize,
}

impl Player {
    pub fn new(id: Uuid, username: String, x: f64, y: f64) -> Self {
        Self {
            id,
            username,
            x,
            y,
            health: 100.0,
            hunger: 100.0,
            cold: 50.0, // Neutral
            inventory: Inventory::default(),
            active_slot: 0,
        }
    }
}