use config::{BalanceConfig, Config, SurvivalConfig};
use world::{InventorySlot, PlayerState, Settlement, Vitals};
use world::biome::BiomeType;

pub mod combat;

pub struct SurvivalSystem;

impl SurvivalSystem {
    pub fn tick(player: &mut PlayerState, dt: f32, config: &SurvivalConfig, balance: &BalanceConfig, biome: Option<&BiomeType>) {
        if !player.spawned {
            return;
        }

        if config.hunger_enabled {
            Self::update_hunger(player, dt, config);
        }

        if config.temperature_enabled {
            Self::update_temperature(player, dt, config, biome);
        }

        Self::apply_vital_effects(player, dt, balance);
        Self::clamp_vitals(player);
    }

    fn update_hunger(player: &mut PlayerState, dt: f32, config: &SurvivalConfig) {
        let decay = config.hunger_decay_per_second * dt;
        player.vitals.hunger -= decay;
    }

    fn update_temperature(player: &mut PlayerState, dt: f32, config: &SurvivalConfig, biome: Option<&BiomeType>) {
        // Use biome temperature as target if available, otherwise default to 50.0
        let target_temp = biome.map(|b| b.base_temperature()).unwrap_or(50.0);
        let diff = target_temp - player.vitals.temperature;
        let change = diff * config.temperature_convergence_rate * dt;
        player.vitals.temperature += change;
    }

    fn apply_vital_effects(player: &mut PlayerState, dt: f32, balance: &BalanceConfig) {
        if player.vitals.hunger <= 0.0 {
            player.vitals.hp -= 1.0 * dt;
        } else if player.vitals.hunger > 80.0 && player.vitals.hp < player.vitals.max_hp {
            player.vitals.hp += 0.5 * dt;
        }

        if player.vitals.temperature <= 10.0 || player.vitals.temperature >= 90.0 {
            player.vitals.hp -= 2.0 * dt;
        }
    }

    fn clamp_vitals(player: &mut PlayerState) {
        player.vitals.hp = player.vitals.hp.clamp(0.0, player.vitals.max_hp);
        player.vitals.hunger = player.vitals.hunger.clamp(0.0, player.vitals.max_hunger);
        player.vitals.temperature = player
            .vitals
            .temperature
            .clamp(0.0, player.vitals.max_temperature);
    }
}

pub struct MovementSystem;

impl MovementSystem {
    pub fn apply_movement(
        player: &mut PlayerState,
        dx: f32,
        dy: f32,
        dt: f32,
        balance: &BalanceConfig,
    ) {
        if !player.spawned {
            return;
        }

        let speed = balance.player.base_speed;
        let magnitude = (dx * dx + dy * dy).sqrt();
        if magnitude > 0.0 {
            let normalized_dx = dx / magnitude;
            let normalized_dy = dy / magnitude;
            player.x += normalized_dx * speed * dt;
            player.y += normalized_dy * speed * dt;
            player.stats.steps += 1;
        }
    }
}

pub struct CraftingSystem;

impl CraftingSystem {
    pub fn craft(
        player: &mut PlayerState,
        recipe_id: &str,
        recipes: &std::collections::HashMap<String, config::Recipe>,
    ) -> Result<(), String> {
        let recipe = recipes
            .get(recipe_id)
            .ok_or_else(|| format!("Unknown recipe: {}", recipe_id))?;

        for ingredient in &recipe.ingredients {
            let has_count = player
                .inventory
                .iter()
                .filter_map(|slot| slot.as_ref())
                .filter(|slot| slot.item == ingredient.item)
                .map(|slot| slot.count)
                .sum::<i32>();

            if has_count < ingredient.count {
                return Err(format!(
                    "Insufficient {}: have {}, need {}",
                    ingredient.item, has_count, ingredient.count
                ));
            }
        }

        for ingredient in &recipe.ingredients {
            let mut remaining = ingredient.count;
            for slot in player.inventory.iter_mut() {
                if let Some(ref mut s) = slot {
                    if s.item == ingredient.item {
                        let take = remaining.min(s.count);
                        s.count -= take;
                        remaining -= take;
                        if s.count == 0 {
                            *slot = None;
                        }
                        if remaining == 0 {
                            break;
                        }
                    }
                }
            }
        }

        let output_slot = player
            .inventory
            .iter_mut()
            .find(|slot| slot.is_none())
            .ok_or("Inventory full")?;

        *output_slot = Some(InventorySlot {
            item: recipe.output.clone(),
            count: recipe.count,
        });

        player.stats.crafts += 1;
        Ok(())
    }
}

pub struct DeathSystem;

impl DeathSystem {
    pub fn check_death(player: &mut PlayerState) -> bool {
        if player.vitals.hp <= 0.0 && player.spawned {
            player.spawned = false;
            player.stats.deaths += 1;
            true
        } else {
            false
        }
    }

    pub fn respawn(player: &mut PlayerState, settlement_x: f32, settlement_y: f32) {
        player.spawned = true;
        player.x = settlement_x;
        player.y = settlement_y;
        player.vitals.hp = player.vitals.max_hp;
        player.vitals.hunger = player.vitals.max_hunger * 0.8;
        player.vitals.temperature = 50.0;
    }
}

pub struct BarrierSystem;

impl BarrierSystem {
    pub fn is_in_safe_zone(
        x: f32,
        y: f32,
        settlement: &Settlement,
    ) -> bool {
        let dx = x - settlement.core_x;
        let dy = y - settlement.core_y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance <= settlement.safe_zone_radius
    }

    pub fn can_pvp(attacker: &PlayerState, target: &PlayerState, _world: &world::World) -> bool {
        // Simplified: check if either player is in a safe zone
        // In a full implementation, this would check against actual settlement data
        let _attacker_in_safe_zone = false;
        let _target_in_safe_zone = false;
        
        // For now, allow PvP everywhere (safe zones not fully implemented)
        !_attacker_in_safe_zone && !_target_in_safe_zone
    }
}

pub struct AchievementSystem;

impl AchievementSystem {
    pub fn check_achievements(player: &mut PlayerState) -> Vec<String> {
        let mut new_achievements = Vec::new();

        if player.stats.steps >= 100 && !player.achievements.contains(&"first_steps".to_string()) {
            new_achievements.push("first_steps".to_string());
        }

        if player.stats.crafts >= 1 && !player.achievements.contains(&"first_craft".to_string()) {
            new_achievements.push("first_craft".to_string());
        }

        if player.stats.gathers >= 10
            && !player.achievements.contains(&"gatherer".to_string())
        {
            new_achievements.push("gatherer".to_string());
        }

        for achievement in &new_achievements {
            player.achievements.push(achievement.clone());
            player.xp += 50;
        }

        new_achievements
    }
}
