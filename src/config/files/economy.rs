use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct EconomyConfig {
    pub tax_rate: f32,
}

impl Default for EconomyConfig {
    fn default() -> Self {
        Self { tax_rate: 0.05 }
    }
}
