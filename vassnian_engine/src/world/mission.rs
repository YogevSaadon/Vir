//! Mission system — mission definitions, selection options, context

use serde::{Deserialize, Serialize};
use super::terrain::Terrain;
use super::phase::GamePhase;

/// Mission difficulty levels.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MissionDifficulty {
    Easy,
    Medium,
    Hard,
    None,
}

impl Default for MissionDifficulty {
    fn default() -> Self {
        MissionDifficulty::Medium
    }
}

/// A mission definition loaded from JSON catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionDef {
    pub id: String,
    pub title: String,
    pub description: String,
    pub phase: GamePhase,
    #[serde(default)]
    pub difficulty: MissionDifficulty,
    #[serde(default)]
    pub time_cost: i32,
    #[serde(default)]
    pub terrain: Terrain,
    pub is_companion_story: Option<String>,
    #[serde(default)]
    pub has_special_item: bool,
    pub story_id: String,
    pub continuation_of: Option<String>,
    pub continues_to: Option<Vec<String>>,
    #[serde(default)]
    pub is_required: bool,
    /// Mission level for enemy scaling and loot. Defaults to 1.
    #[serde(default = "default_mission_level")]
    pub mission_level: i32,
    /// Loot table ID for combat rewards.
    #[serde(default)]
    pub loot_table: Option<String>,
}

/// A mission option presented to the player during selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionOption {
    pub story_id: String,
    pub description: String,
    pub difficulty: MissionDifficulty,
    pub time_cost: i32,
    pub is_companion_story: Option<String>,
    pub has_special_item: bool,
}

/// Runtime context for the current mission (controls shop/city access).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionContext {
    pub allows_shop: bool,
    pub allows_city: bool,
    pub current_terrain: Terrain,
}

fn default_mission_level() -> i32 { 1 }

impl Default for MissionContext {
    fn default() -> Self {
        Self {
            allows_shop: true,
            allows_city: true,
            current_terrain: Terrain::Plains,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mission_context_default() {
        let ctx = MissionContext::default();
        assert!(ctx.allows_shop);
        assert!(ctx.allows_city);
        assert_eq!(ctx.current_terrain, Terrain::Plains);
    }

    #[test]
    fn test_mission_def_serde() {
        let json = r#"{
            "id": "m_test",
            "title": "Test Mission",
            "description": "A test.",
            "phase": "phase0_arrival",
            "difficulty": "easy",
            "time_cost": 1,
            "terrain": "forest",
            "story_id": "story_test",
            "is_required": false
        }"#;
        let mission: MissionDef = serde_json::from_str(json).unwrap();
        assert_eq!(mission.id, "m_test");
        assert_eq!(mission.terrain, Terrain::Forest);
        assert_eq!(mission.difficulty, MissionDifficulty::Easy);
    }
}
