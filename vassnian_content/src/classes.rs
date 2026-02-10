//! Class definitions — Knight, Barbarian, etc.

use serde::{Deserialize, Serialize};
use vassnian_engine::character::stats::StatBlock;
use vassnian_engine::character::entity::AttackType;

/// Class category for grouping.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClassCategory {
    Warrior,
    Caster,
    Rogue,
    Healer,
}

/// Position preference for a class.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Position {
    Front,
    Back,
    Flexible,
}

/// Resource system used by a class.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Cooldown,
    Mana,
    ManaCooldown,
}

/// A class definition loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: ClassCategory,
    pub position: Position,
    pub resource_type: ResourceType,
    pub base_stats: StatBlock,
    #[serde(default)]
    pub starting_combat_skills: Vec<String>,
    #[serde(default)]
    pub starting_world_skills: Vec<String>,
    pub portrait: String,
    pub attack_type: AttackType,
    pub flavor_text: String,
}

/// Container for all class definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassesData {
    pub classes: Vec<ClassDef>,
}
