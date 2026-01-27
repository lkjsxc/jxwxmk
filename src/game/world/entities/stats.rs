use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub steps: u64,
    pub kills: u64,
    pub crafts: u64,
    pub gathers: u64,
    pub deaths: u64,
    pub stat_bonuses: HashMap<String, f32>,
}
