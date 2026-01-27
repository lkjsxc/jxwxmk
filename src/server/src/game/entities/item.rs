use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ItemId(pub String);

impl ItemId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    pub item: ItemId,
    pub count: u32,
    pub level: u32,
}

impl ItemStack {
    pub fn new(item: ItemId, count: u32) -> Self {
        Self {
            item,
            count,
            level: 1,
        }
    }
}
