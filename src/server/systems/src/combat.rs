use world::{PlayerState, World};
use config::BalanceConfig;

pub struct CombatSystem;

#[derive(Debug, Clone)]
pub struct AttackResult {
    pub damage: f32,
    pub hit: bool,
    pub critical: bool,
}

impl CombatSystem {
    pub fn can_attack(
        attacker: &PlayerState,
        target: &PlayerState,
        world: &World,
    ) -> bool {
        // Both players must be spawned
        if !attacker.spawned || !target.spawned {
            return false;
        }
        
        // Check safe zones
        if world.is_in_safe_zone(attacker.x, attacker.y) {
            return false;
        }
        
        if world.is_in_safe_zone(target.x, target.y) {
            return false;
        }
        
        // Check distance (melee range: 2.0 units)
        let dx = attacker.x - target.x;
        let dy = attacker.y - target.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        distance <= 2.0
    }
    
    pub fn perform_attack(
        attacker: &mut PlayerState,
        target: &mut PlayerState,
        balance: &BalanceConfig,
    ) -> AttackResult {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        // Base damage calculation
        let base_damage = 5.0;
        let damage_variance = rng.gen_range(0.8..1.2);
        let critical_chance = 0.1; // 10% crit chance
        
        let is_critical = rng.gen_bool(critical_chance);
        let critical_multiplier = if is_critical { 2.0 } else { 1.0 };
        
        let damage = base_damage * damage_variance * critical_multiplier;
        
        // Apply damage
        target.vitals.hp -= damage;
        
        // Update attacker stats
        attacker.stats.kills += 1;
        
        AttackResult {
            damage,
            hit: true,
            critical: is_critical,
        }
    }
    
    pub fn get_attack_cooldown_remaining(
        player: &PlayerState,
        current_time: f32,
    ) -> f32 {
        // Attack cooldown: 0.5 seconds
        let cooldown = 0.5;
        let last_attack = player.stats.kills as f32 * 0.0; // Simplified
        
        let remaining = cooldown - (current_time - last_attack);
        remaining.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use world::PlayerState;
    use uuid::Uuid;
    
    fn create_test_player() -> PlayerState {
        PlayerState::new(Uuid::new_v4(), "TestPlayer".to_string())
    }
    
    #[test]
    fn test_cannot_attack_when_not_spawned() {
        let mut attacker = create_test_player();
        let mut target = create_test_player();
        attacker.spawned = true;
        target.spawned = false;
        
        let world = World::new(12345);
        
        assert!(!CombatSystem::can_attack(&attacker, &target, &world));
    }
    
    #[test]
    fn test_attack_deals_damage() {
        let mut attacker = create_test_player();
        let mut target = create_test_player();
        attacker.spawned = true;
        target.spawned = true;
        
        // Position them close together
        attacker.x = 0.0;
        attacker.y = 0.0;
        target.x = 1.0;
        target.y = 0.0;
        
        let initial_hp = target.vitals.hp;
        let balance = BalanceConfig {
            version: 1,
            player: config::PlayerBalance {
                base_speed: 5.0,
                base_hp: 30.0,
            },
        };
        
        let result = CombatSystem::perform_attack(&mut attacker, &mut target, &balance);
        
        assert!(result.damage > 0.0);
        assert!(target.vitals.hp < initial_hp);
    }
}
