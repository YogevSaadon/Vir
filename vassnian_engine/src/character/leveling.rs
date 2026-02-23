//! XP & Leveling system — JSON-driven level thresholds, stat/skill points on level-up

use serde::{Deserialize, Serialize};

/// Level configuration loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelConfig {
    /// Cumulative XP thresholds per level: index = level, value = XP needed.
    /// e.g. [0, 0, 50, 120, ...] means level 2 needs 50 total XP, level 3 needs 120.
    /// Index 0 is unused, index 1 is the starting level (0 XP).
    pub xp_per_level: Vec<i32>,
    /// Maximum achievable level.
    pub max_level: i32,
}

/// Result of a level-up check.
#[derive(Debug, Clone)]
pub struct LevelUpResult {
    pub new_level: i32,
    pub levels_gained: i32,
    /// Stat points earned (1 per odd level: 1,3,5,7,9,11,13,15,17,19).
    pub stat_points_earned: i32,
    /// Skill points earned (1 per even level: 2,4,6,8,10,12,14,16,18,20).
    pub skill_points_earned: i32,
}

/// Checks if the entity should level up based on current XP.
/// Returns Some(LevelUpResult) if one or more levels were gained.
/// Stat points: 1 per odd level. Skill points: 1 per even level.
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
        // Count stat points (odd levels) and skill points (even levels) gained
        let mut stat_points = 0;
        let mut skill_points = 0;
        for lvl in (current_level + 1)..=new_level {
            if lvl % 2 == 1 {
                stat_points += 1;  // odd levels give stat points
            } else {
                skill_points += 1; // even levels give skill points
            }
        }

        Some(LevelUpResult {
            new_level,
            levels_gained,
            stat_points_earned: stat_points,
            skill_points_earned: skill_points,
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
        // Level 2 is even -> skill point
        assert_eq!(result.stat_points_earned, 0);
        assert_eq!(result.skill_points_earned, 1);
    }

    #[test]
    fn test_multi_level_up() {
        let config = test_config();
        // 250 XP >= 220 (level 4) but < 360 (level 5)
        // Levels gained: 2, 3, 4
        // Level 2 (even) = skill, Level 3 (odd) = stat, Level 4 (even) = skill
        let result = check_level_up(1, 250, &config).unwrap();
        assert_eq!(result.new_level, 4);
        assert_eq!(result.levels_gained, 3);
        assert_eq!(result.stat_points_earned, 1);  // level 3
        assert_eq!(result.skill_points_earned, 2);  // levels 2 and 4
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
        assert_eq!(xp_for_next_level(1, &config), 50);
        assert_eq!(xp_for_next_level(4, &config), 360);
        assert_eq!(xp_for_next_level(5, &config), -1);
    }
}
