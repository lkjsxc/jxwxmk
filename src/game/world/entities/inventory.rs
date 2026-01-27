use serde::{Deserialize, Serialize};

use super::item::Item;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub slots: Vec<Option<Item>>,
    pub active_slot: usize,
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![None; size],
            active_slot: 0,
        }
    }

    pub fn has_item(&self, kind: &str, count: u32) -> bool {
        let mut remaining = count;
        for slot in &self.slots {
            if let Some(item) = slot {
                if item.kind == kind {
                    if item.amount >= remaining {
                        return true;
                    }
                    remaining -= item.amount;
                }
            }
        }
        false
    }

    pub fn consume_item(&mut self, kind: &str, count: u32) -> bool {
        if !self.has_item(kind, count) {
            return false;
        }
        let mut remaining = count;
        for slot in &mut self.slots {
            if remaining == 0 {
                break;
            }
            if let Some(item) = slot {
                if item.kind == kind {
                    let take = remaining.min(item.amount);
                    item.amount -= take;
                    remaining -= take;
                    if item.amount == 0 {
                        *slot = None;
                    }
                }
            }
        }
        true
    }

    pub fn add_item(&mut self, mut item: Item) {
        for slot in &mut self.slots {
            if let Some(existing) = slot {
                if existing.kind == item.kind && existing.amount < existing.max_stack {
                    let space = existing.max_stack - existing.amount;
                    let add = space.min(item.amount);
                    existing.amount += add;
                    item.amount -= add;
                    if item.amount == 0 {
                        return;
                    }
                }
            }
        }
        for slot in &mut self.slots {
            if slot.is_none() {
                *slot = Some(item);
                return;
            }
        }
    }

    pub fn swap_slots(&mut self, from: usize, to: usize) {
        if from >= self.slots.len() || to >= self.slots.len() {
            return;
        }
        self.slots.swap(from, to);
    }
}
