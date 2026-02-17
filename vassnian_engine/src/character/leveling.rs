//! XP & Leveling system — JSON-driven level thresholds, stat points on level-up

use serde::{Deserialize, Serialize};

/// Level configuration loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelConfig {
    /// Cumulative XP thresholds per level: index = level, value = XP needed.
    /// e.g. [0, 0, 50, 120, ...] means level 2 needs 50 total XP, level 3 needs 120.
    /// Index 0 is unused, index 1 is the starting level (0 XP).
    pub xp_per_level: Vec<i32>,
    /// Stat points awarded per level-up.
    pub stat_points_per_level: i32,
    /// Maximum achievable level.
    pub max_level: i32,
}

/// Result of a level-up check.
#[derive(Debug, Clone)]
pub struct LevelUpResult {
    pub new_level: i32,
    pub levels_gained: i32,
    pub stat_points_earned: i32,
}

/// Checks if the entity should level up based on current XP.
/// Returns Some(LevelUpResult) if one or more levels were gained.
pub fn check_level_up(
    current_level: i32,
    current_exp: i32,
    config: &LevelConfig,
) -> Option<LevelUpResult> {
    let mut new_level = current_level;

    while new_level < config.max_level {
        let next = (new_level + 1) as usize;
        if next >= config.xp_per_level.len() {
            break;
        }
        if current_exp >= config.xp_per_level[next] {
            new_level += 1;
        } else {
            break;
        }
    }

    let levels_gained = new_level - current_level;
    if levels_gained > 0 {
        Some(LevelUpResult {
            new_level,
            levels_gained,
            stat_points_earned: levels_gained * config.stat_points_per_level,
        })
    } else {
        None
    }
}

/// Returns the XP needed for the next level (cumulative threshold).
/// Returns -1 if already at max level.
pub fn xp_for_next_level(current_level: i32, config: &LevelConfig) -> i32 {
    let next = (current_level + 1) as usize;
    if current_level >= config.max_level || next >= config.xp_per_level.len() {
        -1
    } else {
        config.xp_per_level[next]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // index: 0    1    2    3    4    5
    // level: -    1    2    3    4    5
    // XP:    0    0   50  120  220  360
    fn test_config() -> LevelConfig {
        LevelConfig {
            xp_per_level: vec![0, 0, 50, 120, 220, 360],
            stat_points_per_level: 2,
            max_level: 5,
        }
    }

    #[test]
    fn test_no_level_up() {
        let config = test_config();
        // 30 XP is not enough for level 2 (needs 50)
        assert!(check_level_up(1, 30, &config).is_none());
    }

    #[test]
    fn test_single_level_up() {
        let config = test_config();
        // 50 XP = exactly level 2 threshold
        let result = check_level_up(1, 50, &config).unwrap();
        assert_eq!(result.new_level, 2);
        assert_eq!(result.levels_gained, 1);
        assert_eq!(result.stat_points_earned, 2);
    }

    #[test]
    fn test_multi_level_up() {
        let config = test_config();
        // 250 XP >= 220 (level 4) but < 360 (level 5)
        let result = check_level_up(1, 250, &config).unwrap();
        assert_eq!(result.new_level, 4);
        assert_eq!(result.levels_gained, 3);
        assert_eq!(result.stat_points_earned, 6);
    }

    #[test]
    fn test_max_level_cap() {
        let config = test_config();
        let result = check_level_up(1, 99999, &config).unwrap();
        assert_eq!(result.new_level, 5);
    }

    #[test]
    fn test_already_max_level() {
        let config = test_config();
        assert!(check_level_up(5, 99999, &config).is_none());
    }

    #[test]
    fn test_xp_for_next_level() {
        let config = test_config();
        // Level 1 -> 2 needs 50 XP (index 2)
        assert_eq!(xp_for_next_level(1, &config), 50);
        // Level 4 -> 5 needs 360 XP (index 5)
        assert_eq!(xp_for_next_level(4, &config), 360);
        // Level 5 = max, no next
        assert_eq!(xp_for_next_level(5, &config), -1);
    }
}
