//! Enemy scaling — scales stats and rewards based on mission level

use serde::{Deserialize, Serialize};
use crate::character::stats::StatBlock;

/// Scaling configuration loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingConfig {
    /// Stat growth per level difference (e.g. 0.12 = +12% per level).
    pub stat_growth_per_level: f32,
    /// HP growth per level difference.
    pub hp_growth_per_level: f32,
    /// Reward (gold/exp) growth per level difference.
    pub reward_growth_per_level: f32,
}

impl Default for ScalingConfig {
    fn default() -> Self {
        Self {
            stat_growth_per_level: 0.12,
            hp_growth_per_level: 0.15,
            reward_growth_per_level: 0.10,
        }
    }
}

/// Scales a stat block from base_level to target_level.
/// Returns a new StatBlock with scaled values.
pub fn scale_enemy_stats(
    base: &StatBlock,
    base_level: i32,
    target_level: i32,
    config: &ScalingConfig,
) -> StatBlock {
    let level_diff = target_level - base_level;
    if level_diff <= 0 {
        return base.clone();
    }

    let stat_mult = 1.0 + config.stat_growth_per_level * level_diff as f32;
    let hp_mult = 1.0 + config.hp_growth_per_level * level_diff as f32;

    StatBlock {
        strength: (base.strength as f32 * stat_mult).round() as i32,
        vitality: (base.vitality as f32 * hp_mult).round() as i32,
        intelligence: (base.intelligence as f32 * stat_mult).round() as i32,
        faith: (base.faith as f32 * stat_mult).round() as i32,
        speed: (base.speed as f32 * stat_mult).round() as i32,
        dexterity: (base.dexterity as f32 * stat_mult).round() as i32,
        luck: (base.luck as f32 * stat_mult).round() as i32,
    }
}

/// Scales rewards (exp, gold) from base_level to target_level.
pub fn scale_rewards(
    base_exp: i32,
    base_gold: i32,
    base_level: i32,
    target_level: i32,
    config: &ScalingConfig,
) -> (i32, i32) {
    let level_diff = target_level - base_level;
    if level_diff <= 0 {
        return (base_exp, base_gold);
    }

    let mult = 1.0 + config.reward_growth_per_level * level_diff as f32;
    (
        (base_exp as f32 * mult).round() as i32,
        (base_gold as f32 * mult).round() as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_scaling_same_level() {
        let config = ScalingConfig::default();
        let base = StatBlock::all(10);
        let scaled = scale_enemy_stats(&base, 1, 1, &config);
        assert_eq!(scaled, base);
    }

    #[test]
    fn test_no_scaling_lower_level() {
        let config = ScalingConfig::default();
        let base = StatBlock::all(10);
        let scaled = scale_enemy_stats(&base, 5, 3, &config);
        assert_eq!(scaled, base);
    }

    #[test]
    fn test_scaling_up() {
        let config = ScalingConfig::default();
        let base = StatBlock { strength: 10, vitality: 10, ..StatBlock::all(5) };
        let scaled = scale_enemy_stats(&base, 1, 3, &config);
        // strength: 10 * (1 + 0.12*2) = 10 * 1.24 = 12.4 -> 12
        assert_eq!(scaled.strength, 12);
        // vitality uses hp_growth: 10 * (1 + 0.15*2) = 10 * 1.30 = 13
        assert_eq!(scaled.vitality, 13);
    }

    #[test]
    fn test_reward_scaling() {
        let config = ScalingConfig::default();
        let (exp, gold) = scale_rewards(10, 5, 1, 3, &config);
        // 10 * (1 + 0.10*2) = 10 * 1.20 = 12
        assert_eq!(exp, 12);
        // 5 * 1.20 = 6
        assert_eq!(gold, 6);
    }

    #[test]
    fn test_reward_no_scaling() {
        let config = ScalingConfig::default();
        let (exp, gold) = scale_rewards(10, 5, 1, 1, &config);
        assert_eq!(exp, 10);
        assert_eq!(gold, 5);
    }
}
