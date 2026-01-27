use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub kind: String,
    pub amount: u32,
    pub max_stack: u32,
    pub level: u8,
    pub quality: u8,
    pub xp: u32,
}

impl Item {
    pub fn new(kind: impl Into<String>, amount: u32) -> Self {
        Self {
            kind: kind.into(),
            amount,
            max_stack: 99,
            level: 1,
            quality: 0,
            xp: 0,
        }
    }
}
