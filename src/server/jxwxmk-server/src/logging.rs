use log::info;
use std::collections::HashMap;

pub struct StructuredLogger;

impl StructuredLogger {
    pub fn log_event(
        event: &str,
        context: HashMap<String, String>,
    ) {
        let context_str = context
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(", ");
        
        info!("event={}, {}", event, context_str);
    }

    pub fn player_connected(player_id: &str, session_token: &str) {
        let mut context = HashMap::new();
        context.insert("player_id".to_string(), player_id.to_string());
        context.insert("session_token".to_string(), session_token.to_string());
        Self::log_event("player_connected", context);
    }

    pub fn player_disconnected(player_id: &str) {
        let mut context = HashMap::new();
        context.insert("player_id".to_string(), player_id.to_string());
        Self::log_event("player_disconnected", context);
    }

    pub fn player_spawned(player_id: &str, x: f32, y: f32) {
        let mut context = HashMap::new();
        context.insert("player_id".to_string(), player_id.to_string());
        context.insert("x".to_string(), x.to_string());
        context.insert("y".to_string(), y.to_string());
        Self::log_event("player_spawned", context);
    }

    pub fn player_died(player_id: &str) {
        let mut context = HashMap::new();
        context.insert("player_id".to_string(), player_id.to_string());
        Self::log_event("player_died", context);
    }

    pub fn chunk_loaded(cx: i32, cy: i32) {
        let mut context = HashMap::new();
        context.insert("cx".to_string(), cx.to_string());
        context.insert("cy".to_string(), cy.to_string());
        Self::log_event("chunk_loaded", context);
    }

    pub fn chunk_unloaded(cx: i32, cy: i32) {
        let mut context = HashMap::new();
        context.insert("cx".to_string(), cx.to_string());
        context.insert("cy".to_string(), cy.to_string());
        Self::log_event("chunk_unloaded", context);
    }

    pub fn settlement_generated(settlement_id: &str, name: &str, x: f32, y: f32) {
        let mut context = HashMap::new();
        context.insert("settlement_id".to_string(), settlement_id.to_string());
        context.insert("name".to_string(), name.to_string());
        context.insert("x".to_string(), x.to_string());
        context.insert("y".to_string(), y.to_string());
        Self::log_event("settlement_generated", context);
    }

    pub fn achievement_unlocked(player_id: &str, achievement_id: &str) {
        let mut context = HashMap::new();
        context.insert("player_id".to_string(), player_id.to_string());
        context.insert("achievement_id".to_string(), achievement_id.to_string());
        Self::log_event("achievement_unlocked", context);
    }

    pub fn item_crafted(player_id: &str, recipe_id: &str) {
        let mut context = HashMap::new();
        context.insert("player_id".to_string(), player_id.to_string());
        context.insert("recipe_id".to_string(), recipe_id.to_string());
        Self::log_event("item_crafted", context);
    }

    pub fn tick_completed(duration_ms: u64, player_count: usize) {
        let mut context = HashMap::new();
        context.insert("duration_ms".to_string(), duration_ms.to_string());
        context.insert("player_count".to_string(), player_count.to_string());
        Self::log_event("tick_completed", context);
    }
}
