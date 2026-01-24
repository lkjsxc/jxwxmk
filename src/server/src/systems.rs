use serde::{Serialize, Deserialize};
use crate::world::{ResourceNode, ResourceType, Inventory};

pub struct MovementSystem;

impl MovementSystem {
    pub fn update_position(
        position: &mut (f32, f32),
        velocity: (f32, f32),
        delta_time: f32,
        speed_multiplier: f32,
    ) {
        position.0 += velocity.0 * delta_time * speed_multiplier;
        position.1 += velocity.1 * delta_time * speed_multiplier;
    }
    
    pub fn calculate_velocity(direction: (f32, f32), speed: f32, sprinting: bool) -> (f32, f32) {
        let sprint_multiplier = if sprinting { 1.8 } else { 1.0 };
        let effective_speed = speed * sprint_multiplier;
        
        // Normalize direction vector
        let magnitude = (direction.0.powi(2) + direction.1.powi(2)).sqrt();
        if magnitude > 0.0 {
            let normalized = (direction.0 / magnitude, direction.1 / magnitude);
            (normalized.0 * effective_speed, normalized.1 * effective_speed)
        } else {
            (0.0, 0.0)
        }
    }
}

pub struct CombatSystem;

impl CombatSystem {
    pub fn calculate_damage(
        attacker_damage: f32,
        defender_armor: f32,
        attack_type: AttackType,
    ) -> f32 {
        let armor_reduction = match attack_type {
            AttackType::Melee => defender_armor * 0.7,
            AttackType::Ranged => defender_armor * 0.5,
            AttackType::Magic => defender_armor * 0.3,
        };
        
        (attacker_damage - armor_reduction).max(1.0)
    }
    
    pub fn apply_damage(health: &mut f32, damage: f32) -> bool {
        *health -= damage;
        if *health <= 0.0 {
            *health = 0.0;
            true // Entity died
        } else {
            false
        }
    }
}

pub struct SurvivalSystem;

impl SurvivalSystem {
    pub fn update_hunger(current: &mut f32, max: f32, delta_time: f32, rate: f32) {
        *current -= rate * delta_time;
        *current = current.clamp(0.0, max);
    }
    
    pub fn update_thirst(current: &mut f32, max: f32, delta_time: f32, rate: f32) {
        *current -= rate * delta_time;
        *current = current.clamp(0.0, max);
    }
    
    pub fn update_health(
        current: &mut f32,
        max: f32,
        hunger: f32,
        thirst: f32,
        delta_time: f32,
    ) {
        let hunger_factor = (hunger / max).min(1.0);
        let thirst_factor = (thirst / max).min(1.0);
        
        let health_change = if hunger_factor > 0.7 && thirst_factor > 0.7 {
            0.1 * delta_time
        } else if hunger_factor < 0.3 || thirst_factor < 0.3 {
            -0.2 * delta_time
        } else {
            0.0
        };
        
        *current += health_change;
        *current = current.clamp(0.0, max);
    }
}

pub struct ResourceSystem;

impl ResourceSystem {
    pub fn get_resource_item_id(resource_type: &ResourceType) -> String {
        match resource_type {
            ResourceType::Tree => "wood".to_string(),
            ResourceType::Rock => "stone".to_string(),
            ResourceType::Bush => "berry".to_string(),
            ResourceType::Ore => "ore".to_string(),
            ResourceType::Water => "water".to_string(),
        }
    }

    pub fn gather(
        resource: &mut ResourceNode,
        inventory: &mut Inventory,
        gather_power: f32,
    ) -> Option<(String, u32)> {
        if resource.quantity <= 0.0 {
            return None;
        }

        let amount_to_take = gather_power.min(resource.quantity);
        resource.quantity -= amount_to_take;
        
        if resource.quantity <= 0.0 {
            resource.quantity = 0.0;
        }
        
        let item_id = Self::get_resource_item_id(&resource.resource_type);
        let amount_int = amount_to_take.ceil() as u32;

        *inventory.items.entry(item_id.clone()).or_insert(0) += amount_int;
        
        Some((item_id, amount_int))
    }
}

pub struct CraftingSystem;

impl CraftingSystem {
    pub fn can_craft(recipe: &CraftingRecipe, inventory: &Inventory) -> bool {
        recipe.requirements.iter().all(|(item_type, quantity)| {
            inventory.items.get(item_type).map_or(false, |&qty| qty >= *quantity)
        })
    }
    
    pub fn perform_craft(recipe: &CraftingRecipe, inventory: &mut Inventory) -> bool {
        if !Self::can_craft(recipe, inventory) {
            return false;
        }
        
        for (item_type, quantity) in &recipe.requirements {
            if let Some(item_quantity) = inventory.items.get_mut(item_type) {
                *item_quantity -= *quantity;
            }
        }
        
        *inventory.items.entry(recipe.result.item_type.clone()).or_insert(0) += recipe.result.quantity;
        true
    }

    pub fn get_default_recipes() -> Vec<CraftingRecipe> {
        vec![
            CraftingRecipe {
                id: "wood_pickaxe".to_string(),
                name: "Wooden Pickaxe".to_string(),
                requirements: vec![("wood".to_string(), 10)],
                result: CraftingResult { item_type: "pickaxe_wood".to_string(), quantity: 1 },
                crafting_time: 2.0,
                tier: 1,
            },
            CraftingRecipe {
                id: "stone_pickaxe".to_string(),
                name: "Stone Pickaxe".to_string(),
                requirements: vec![("wood".to_string(), 5), ("stone".to_string(), 10)],
                result: CraftingResult { item_type: "pickaxe_stone".to_string(), quantity: 1 },
                crafting_time: 3.0,
                tier: 1,
            },
            CraftingRecipe {
                id: "campfire".to_string(),
                name: "Campfire".to_string(),
                requirements: vec![("wood".to_string(), 20), ("stone".to_string(), 5)],
                result: CraftingResult { item_type: "campfire".to_string(), quantity: 1 },
                crafting_time: 5.0,
                tier: 1,
            },
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingRecipe {
    pub id: String,
    pub name: String,
    pub requirements: Vec<(String, u32)>,
    pub result: CraftingResult,
    pub crafting_time: f32,
    pub tier: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingResult {
    pub item_type: String,
    pub quantity: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum AttackType {
    Melee,
    Ranged,
    Magic,
}