//! Damage formulas — attack vs defense, crits, equipment bonuses

use rand::Rng;
use crate::character::stats::DerivedStats;

/// Result of a damage calculation.
#[derive(Debug, Clone)]
pub struct DamageResult {
    /// Final damage after all modifiers.
    pub damage: i32,
    /// Whether this was a critical hit.
    pub is_crit: bool,
    /// Whether the attack was evaded.
    pub is_evade: bool,
}

/// Calculates physical melee damage from attacker to defender.
pub fn calculate_melee_damage(
    attacker: &DerivedStats,
    defender: &DerivedStats,
) -> DamageResult {
    let mut rng = rand::thread_rng();

    // Check evasion
    let evade_roll: f32 = rng.gen();
    if evade_roll < defender.evasion {
        return DamageResult {
            damage: 0,
            is_crit: false,
            is_evade: true,
        };
    }

    // Base damage = physical_damage - defense (min 1)
    let base_damage = (attacker.physical_damage - defender.defense).max(1);

    // Variance: +/- 15%
    let variance: f32 = rng.gen_range(0.85..1.15);
    let mut final_damage = (base_damage as f32 * variance) as i32;

    // Check crit
    let crit_roll: f32 = rng.gen();
    let is_crit = crit_roll < attacker.crit_chance;
    if is_crit {
        final_damage = (final_damage as f32 * 1.5) as i32;
    }

    // Minimum 1 damage
    final_damage = final_damage.max(1);

    DamageResult {
        damage: final_damage,
        is_crit,
        is_evade: false,
    }
}

/// Calculates potion damage (flat, ignores defense).
pub fn calculate_potion_damage(base_amount: i32) -> i32 {
    base_amount
}

/// Calculates healing amount (flat for MVP).
pub fn calculate_healing(base_amount: i32) -> i32 {
    base_amount
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::stats::StatBlock;

    #[test]
    fn test_melee_damage_positive() {
        let attacker_stats = StatBlock { strength: 5, ..StatBlock::all(1) };
        let defender_stats = StatBlock { vitality: 1, ..StatBlock::all(1) };
        let attacker = DerivedStats::from_stats(&attacker_stats);
        let defender = DerivedStats::from_stats(&defender_stats);

        // Run multiple times to account for RNG
        let mut total_damage = 0;
        for _ in 0..100 {
            let result = calculate_melee_damage(&attacker, &defender);
            if !result.is_evade {
                assert!(result.damage >= 1);
                total_damage += result.damage;
            }
        }
        assert!(total_damage > 0);
    }

    #[test]
    fn test_minimum_damage() {
        let attacker_stats = StatBlock::all(1);
        let defender_stats = StatBlock { vitality: 10, ..StatBlock::all(1) };
        let attacker = DerivedStats::from_stats(&attacker_stats);
        let defender = DerivedStats::from_stats(&defender_stats);

        let result = calculate_melee_damage(&attacker, &defender);
        if !result.is_evade {
            assert!(result.damage >= 1);
        }
    }
}
