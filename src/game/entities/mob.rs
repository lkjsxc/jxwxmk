use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MobType {
    Rabbit,
    Wolf,
    Bear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mob {
    pub id: Uuid,
    pub m_type: MobType,
    pub x: f64,
    pub y: f64,
    pub health: f64,
    pub level: u32,
    pub target_id: Option<Uuid>, // For aggression
}

use crate::game::config::MobConfig;

impl Mob {
    pub fn new(m_type: MobType, x: f64, y: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            m_type,
            x,
            y,
            health: 10.0, // Default, should use new_with_config
            level: 1,
            target_id: None,
        }
    }

    pub fn new_with_config(m_type: MobType, x: f64, y: f64, cfg: &MobConfig) -> Self {
        let health = match m_type {
            MobType::Rabbit => cfg.rabbit_health,
            MobType::Wolf => cfg.wolf_health,
            MobType::Bear => cfg.bear_health,
        };
        Self {
            id: Uuid::new_v4(),
            m_type,
            x,
            y,
            health,
            level: 1,
            target_id: None,
        }
    }
}
