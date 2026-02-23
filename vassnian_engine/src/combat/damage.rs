//! Damage formulas — LoL-style armor/MR, shield block, no crits/evasion

use rand::Rng;

/// Result of a damage calculation.
#[derive(Debug, Clone)]
pub struct DamageResult {
    /// Final damage after all modifiers.
    pub damage: i32,
}

/// Calculates physical damage with LoL-style armor reduction.
/// Shield subtracts flat damage before armor applies.
/// Armor formula: reduction = armor / (armor + 10)
pub fn calculate_physical_damage(
    raw_damage: i32,
    target_armor: i32,
    target_shield: i32,
) -> DamageResult {
    let mut rng = rand::thread_rng();

    // Shield reduces raw damage first
    let after_shield = (raw_damage - target_shield).max(0);

    // Armor reduces with diminishing returns: reduction = armor / (armor + 10)
    let armor = target_armor.max(0) as f32;
    let reduction = armor / (armor + 10.0);
    let after_armor = after_shield as f32 * (1.0 - reduction);

    // ±10% variance
    let variance: f32 = rng.gen_range(0.90..1.10);
    let final_damage = (after_armor * variance) as i32;

    DamageResult {
        damage: final_damage.max(1),
    }
}

/// Calculates magic damage with MR reduction (same formula as armor).
pub fn calculate_magic_damage(
    raw_damage: i32,
    target_mr: i32,
) -> DamageResult {
    let mut rng = rand::thread_rng();

    let mr = target_mr.max(0) as f32;
    let reduction = mr / (mr + 10.0);
    let after_mr = raw_damage as f32 * (1.0 - reduction);

    // ±10% variance
    let variance: f32 = rng.gen_range(0.90..1.10);
    let final_damage = (after_mr * variance) as i32;

    DamageResult {
        damage: final_damage.max(1),
    }
}

/// Calculates potion damage (flat, ignores defense).
pub fn calculate_potion_damage(base_amount: i32) -> i32 {
    base_amount
}

/// Calculates healing amount (flat for now).
pub fn calculate_healing(base_amount: i32) -> i32 {
    base_amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_damage_no_armor() {
        // 10 raw, 0 armor, 0 shield -> ~10 damage (±10%)
        let mut total = 0;
        for _ in 0..100 {
            let result = calculate_physical_damage(10, 0, 0);
            assert!(result.damage >= 1);
            total += result.damage;
        }
        let avg = total as f32 / 100.0;
        assert!(avg > 8.0 && avg < 12.0);
    }

    #[test]
    fn test_physical_damage_with_armor() {
        // 10 raw, 10 armor = 50% reduction -> ~5 damage
        let mut total = 0;
        for _ in 0..100 {
            let result = calculate_physical_damage(10, 10, 0);
            total += result.damage;
        }
        let avg = total as f32 / 100.0;
        assert!(avg > 3.5 && avg < 6.5);
    }

    #[test]
    fn test_shield_reduces_before_armor() {
        // 20 raw, 3 shield, 10 armor
        // After shield: 17. After armor (50%): ~8.5
        let mut total = 0;
        for _ in 0..100 {
            let result = calculate_physical_damage(20, 10, 3);
            total += result.damage;
        }
        let avg = total as f32 / 100.0;
        assert!(avg > 6.0 && avg < 11.0);
    }

    #[test]
    fn test_minimum_damage() {
        // Very low raw vs high armor -> still 1 minimum
        let result = calculate_physical_damage(1, 30, 0);
        assert!(result.damage >= 1);
    }

    #[test]
    fn test_magic_damage_with_mr() {
        // 10 raw, 10 MR = 50% reduction -> ~5 damage
        let mut total = 0;
        for _ in 0..100 {
            let result = calculate_magic_damage(10, 10);
            total += result.damage;
        }
        let avg = total as f32 / 100.0;
        assert!(avg > 3.5 && avg < 6.5);
    }
}
