use systems::*;
use world::{InventorySlot, PlayerState, Vitals};
use config::{BalanceConfig, PlayerBalance, SurvivalConfig};
use std::collections::HashMap;

#[test]
fn test_survival_hunger_decay() {
    let mut player = create_test_player();
    let survival = create_test_survival_config();
    let balance = create_test_balance_config();
    
    let initial_hunger = player.vitals.hunger;
    SurvivalSystem::tick(&mut player,
        1.0,
        &survival,
        &balance,
    );
    
    assert!(player.vitals.hunger < initial_hunger);
    assert_eq!(player.vitals.hunger, initial_hunger - 0.5);
}

#[test]
fn test_survival_temperature_convergence() {
    let mut player = create_test_player();
    player.vitals.temperature = 30.0;
    let survival = create_test_survival_config();
    let balance = create_test_balance_config();
    
    SurvivalSystem::tick(&mut player,
        1.0,
        &survival,
        &balance,
    );
    
    assert!(player.vitals.temperature > 30.0);
    assert!(player.vitals.temperature < 50.0);
}

#[test]
fn test_survival_vitals_clamped() {
    let mut player = create_test_player();
    player.vitals.hunger = -10.0;
    player.vitals.hp = 100.0;
    let survival = create_test_survival_config();
    let balance = create_test_balance_config();
    
    SurvivalSystem::tick(&mut player,
        1.0,
        &survival,
        &balance,
    );
    
    assert_eq!(player.vitals.hunger, 0.0);
    assert_eq!(player.vitals.hp, player.vitals.max_hp);
}

#[test]
fn test_movement_applies_correctly() {
    let mut player = create_test_player();
    player.spawned = true;
    let balance = create_test_balance_config();
    
    let initial_x = player.x;
    let initial_y = player.y;
    let initial_steps = player.stats.steps;
    
    MovementSystem::apply_movement(
        &mut player,
        1.0,
        0.0,
        1.0,
        &balance,
    );
    
    assert!(player.x > initial_x);
    assert_eq!(player.y, initial_y);
    assert_eq!(player.stats.steps, initial_steps + 1);
}

#[test]
fn test_movement_no_movement_when_not_spawned() {
    let mut player = create_test_player();
    player.spawned = false;
    let balance = create_test_balance_config();
    
    let initial_x = player.x;
    let initial_steps = player.stats.steps;
    
    MovementSystem::apply_movement(
        &mut player,
        1.0,
        0.0,
        1.0,
        &balance,
    );
    
    assert_eq!(player.x, initial_x);
    assert_eq!(player.stats.steps, initial_steps);
}

#[test]
fn test_crafting_consumes_ingredients() {
    let mut player = create_test_player();
    player.spawned = true;
    player.inventory[0] = Some(InventorySlot {
        item: "wood".to_string(),
        count: 10,
    });
    
    let mut recipes = HashMap::new();
    recipes.insert(
        "wood_pickaxe".to_string(),
        config::Recipe {
            ingredients: vec![config::Ingredient {
                item: "wood".to_string(),
                count: 10,
            }],
            output: "wood_pickaxe".to_string(),
            count: 1,
            station_required: None,
        },
    );
    
    let result = CraftingSystem::craft(
        &mut player,
        "wood_pickaxe",
        &recipes,
    );
    
    assert!(result.is_ok());
    assert!(player.inventory[0].is_none());
    assert!(player.inventory.iter().any(|slot| {
        slot.as_ref().map_or(false, |s| s.item == "wood_pickaxe")
    }));
    assert_eq!(player.stats.crafts, 1);
}

#[test]
fn test_crafting_fails_without_ingredients() {
    let mut player = create_test_player();
    let recipes = HashMap::new();
    
    let result = CraftingSystem::craft(
        &mut player,
        "wood_pickaxe",
        &recipes,
    );
    
    assert!(result.is_err());
}

#[test]
fn test_death_detected_when_hp_zero() {
    let mut player = create_test_player();
    player.spawned = true;
    player.vitals.hp = 0.0;
    
    let died = DeathSystem::check_death(&mut player);
    
    assert!(died);
    assert!(!player.spawned);
    assert_eq!(player.stats.deaths, 1);
}

#[test]
fn test_death_not_detected_when_hp_positive() {
    let mut player = create_test_player();
    player.spawned = true;
    player.vitals.hp = 10.0;
    
    let died = DeathSystem::check_death(&mut player);
    
    assert!(!died);
    assert!(player.spawned);
    assert_eq!(player.stats.deaths, 0);
}

#[test]
fn test_respawn_resets_vitals() {
    let mut player = create_test_player();
    player.vitals.hp = 0.0;
    player.vitals.hunger = 0.0;
    player.vitals.temperature = 20.0;
    
    DeathSystem::respawn(&mut player,
        100.0,
        100.0,
    );
    
    assert!(player.spawned);
    assert_eq!(player.x, 100.0);
    assert_eq!(player.y, 100.0);
    assert_eq!(player.vitals.hp, player.vitals.max_hp);
    assert_eq!(player.vitals.hunger, player.vitals.max_hunger * 0.8);
    assert_eq!(player.vitals.temperature, 50.0);
}

#[test]
fn test_achievement_first_steps() {
    let mut player = create_test_player();
    player.stats.steps = 100;
    
    let new_achievements = AchievementSystem::check_achievements(&mut player,
    );
    
    assert!(new_achievements.contains(&"first_steps".to_string()));
    assert!(player.achievements.contains(&"first_steps".to_string()));
    assert_eq!(player.xp, 50);
}

#[test]
fn test_achievement_first_craft() {
    let mut player = create_test_player();
    player.stats.crafts = 1;
    
    let new_achievements = AchievementSystem::check_achievements(&mut player,
    );
    
    assert!(new_achievements.contains(&"first_craft".to_string()));
    assert!(player.achievements.contains(&"first_craft".to_string()));
}

#[test]
fn test_achievement_no_duplicate() {
    let mut player = create_test_player();
    player.stats.steps = 100;
    player.achievements.push("first_steps".to_string());
    
    let new_achievements = AchievementSystem::check_achievements(&mut player,
    );
    
    assert!(!new_achievements.contains(&"first_steps".to_string()));
}

// Helper functions

fn create_test_player() -> PlayerState {
    use uuid::Uuid;
    PlayerState::new(Uuid::new_v4(), "TestPlayer".to_string())
}

fn create_test_survival_config() -> SurvivalConfig {
    SurvivalConfig {
        version: 1,
        hunger_enabled: true,
        hunger_decay_per_second: 0.5,
        temperature_enabled: true,
        temperature_convergence_rate: 0.1,
        thirst_enabled: false,
    }
}

fn create_test_balance_config() -> BalanceConfig {
    BalanceConfig {
        version: 1,
        player: PlayerBalance {
            base_speed: 5.0,
            base_hp: 30.0,
        },
    }
}
