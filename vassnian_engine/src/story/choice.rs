//! Choice resolution — stat checks, world skill checks, outcomes

use serde::{Deserialize, Serialize};
use crate::character::stats::StatBlock;

/// Requirement for a story choice to be available.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChoiceRequirement {
    /// Requires a minimum stat value.
    Stat { stat: String, min_value: i32 },
    /// Requires a world skill (player, companion, or any).
    WorldSkill { skill_id: String, source: String },
    /// Requires a tag to be set.
    Tag { tag: String },
    /// Requires an item in inventory.
    Item { item_id: String },
}

/// Combat trigger from a story choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatTrigger {
    pub enemy_group: String,
    pub on_win: String,
    pub on_lose: String,
}

/// A single choice in a story node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryChoice {
    pub text: String,
    pub next_node: String,
    pub requirement: Option<ChoiceRequirement>,
    #[serde(default)]
    pub tags_to_set: Vec<String>,
    pub trigger_combat: Option<CombatTrigger>,
}

/// Rewards from completing a story node.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Rewards {
    #[serde(default)]
    pub gold: i32,
    #[serde(default)]
    pub items: Vec<String>,
    #[serde(default)]
    pub exp: i32,
}

/// Checks if a stat requirement is met by the given stats.
pub fn check_stat_requirement(
    stats: &StatBlock,
    stat_name: &str,
    min_value: i32,
) -> bool {
    stats.get(stat_name).map_or(false, |v| v >= min_value)
}

/// Checks if a world skill requirement is met.
pub fn check_world_skill(
    player_skills: &[String],
    companion_skills: &[String],
    skill_id: &str,
    source: &str,
) -> (bool, Option<String>) {
    match source {
        "player" => {
            let has = player_skills.iter().any(|s| s == skill_id);
            (has, if has { Some("You".to_string()) } else { None })
        }
        "companion" => {
            let has = companion_skills.iter().any(|s| s == skill_id);
            (has, if has { Some("Companion".to_string()) } else { None })
        }
        "any" | _ => {
            if player_skills.iter().any(|s| s == skill_id) {
                (true, Some("You".to_string()))
            } else if companion_skills.iter().any(|s| s == skill_id) {
                (true, Some("Companion".to_string()))
            } else {
                (false, None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_check() {
        let stats = StatBlock { strength: 3, ..StatBlock::all(1) };
        assert!(check_stat_requirement(&stats, "strength", 3));
        assert!(!check_stat_requirement(&stats, "strength", 4));
    }

    #[test]
    fn test_world_skill_check() {
        let player = vec!["climbing".to_string()];
        let companion = vec!["persuasion".to_string()];

        let (met, source) = check_world_skill(&player, &companion, "climbing", "any");
        assert!(met);
        assert_eq!(source, Some("You".to_string()));

        let (met, source) = check_world_skill(&player, &companion, "persuasion", "any");
        assert!(met);
        assert_eq!(source, Some("Companion".to_string()));

        let (met, _) = check_world_skill(&player, &companion, "lockpicking", "any");
        assert!(!met);
    }
}
