use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub target_id: Option<Uuid>, // For aggression
}

impl Mob {
    pub fn new(m_type: MobType, x: f64, y: f64) -> Self {
        let health = match m_type {
            MobType::Rabbit => 10.0,
            MobType::Wolf => 50.0,
            MobType::Bear => 200.0,
        };
        Self {
            id: Uuid::new_v4(),
            m_type,
            x,
            y,
            health,
            target_id: None,
        }
    }
}
