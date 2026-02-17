//! Class definitions — Knight, Barbarian, etc.

use serde::{Deserialize, Serialize};
use vassnian_engine::character::stats::StatBlock;
use vassnian_engine::character::entity::AttackType;
use vassnian_engine::world::Terrain;

/// Class category for grouping.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClassCategory {
    Warrior,
    Caster,
    Rogue,
    Healer,
    Support,
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

/// Sub-choice for starting passive (e.g., Ranger picks terrain).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PassiveSubChoice {
    ChooseTerrain { options: Vec<Terrain> },
    ChooseDeity { options: Vec<String> },
    ChooseWeapon { options: Vec<String> },
}

/// A passive effect granted by a class starting passive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassiveEffect {
    pub effect_type: String,
    pub value: f32,
    pub condition: Option<String>,
}

/// Starting passive for a class — grants exclusive skills and effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassPassive {
    pub id: String,
    pub name: String,
    pub description: String,
    pub sub_choice: Option<PassiveSubChoice>,
    #[serde(default)]
    pub grants_world_skills: Vec<String>,
    #[serde(default)]
    pub grants_combat_skills: Vec<String>,
    #[serde(default)]
    pub combat_effects: Vec<PassiveEffect>,
    #[serde(default)]
    pub world_effects: Vec<PassiveEffect>,
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
    #[serde(default)]
    pub starting_passive: Option<ClassPassive>,
}

/// Container for all class definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassesData {
    pub classes: Vec<ClassDef>,
}
