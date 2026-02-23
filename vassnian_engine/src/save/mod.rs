//! Save system — serialize/deserialize game state

use serde::{Deserialize, Serialize};
use crate::character::stats::StatBlock;
use crate::character::equipment::EquipmentSlots;
use crate::inventory::belt::PotionBelt;
use crate::inventory::items::{PotionStack, EquipmentInstance};
use crate::story::engine::StoryJournal;

/// Complete save data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub player: PlayerSaveData,
    pub companions: Vec<String>,
    pub inventory: InventorySaveData,
    pub world_state: WorldSaveData,
    pub current_story: Option<String>,
    pub current_node: Option<String>,
}

/// Player-specific save data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSaveData {
    pub name: String,
    pub avatar: String,
    pub class: String,
    pub level: i32,
    pub exp: i32,
    pub stats: StatBlock,
    pub combat_skills: Vec<String>,
    pub world_skills: Vec<String>,
    pub equipped: EquipmentSlots,
    #[serde(default)]
    pub pending_stat_points: i32,
    #[serde(default)]
    pub pending_skill_points: i32,
}

/// Inventory save data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySaveData {
    pub gold: i32,
    pub potions: Vec<PotionStack>,
    pub equipment: Vec<EquipmentInstance>,
    pub potion_belt: PotionBelt,
}

/// World state save data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSaveData {
    pub tags: Vec<String>,
    pub completed_stories: Vec<String>,
    pub available_stories: Vec<String>,
    pub world_level: i32,
    #[serde(default)]
    pub journal: StoryJournal,
}

impl SaveData {
    /// Serializes save data to JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserializes save data from JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Writes save data to a file path.
pub fn save_to_file(data: &SaveData, path: &std::path::Path) -> Result<(), String> {
    let json = data.to_json().map_err(|e| format!("Serialize error: {}", e))?;
    std::fs::write(path, json).map_err(|e| format!("Write error: {}", e))
}

/// Loads save data from a file path.
pub fn load_from_file(path: &std::path::Path) -> Result<SaveData, String> {
    let json = std::fs::read_to_string(path)
        .map_err(|e| format!("Read error: {}", e))?;
    SaveData::from_json(&json).map_err(|e| format!("Parse error: {}", e))
}

/// Checks if a save file exists at the given path.
pub fn save_exists(path: &std::path::Path) -> bool {
    path.exists()
}
