use crate::game::world::entities::Mob;

pub fn calculate_damage(weapon_damage: f32, stat_bonus: f32, level_bonus: f32) -> f32 {
    weapon_damage * (1.0 + stat_bonus + level_bonus)
}

pub fn damage_mob(mob: &mut Mob, damage: f32) {
    mob.health -= damage;
    if mob.health < 0.0 {
        mob.health = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn damage_formula_matches_docs() {
        let damage = calculate_damage(10.0, 0.2, 0.1);
        assert!((damage - 13.0).abs() < f32::EPSILON);
    }
}
