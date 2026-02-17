//! Combat skill definitions

use serde::{Deserialize, Serialize};
use vassnian_engine::character::stats::StatBlock;
use vassnian_engine::inventory::items::TargetType;

/// Skill type — active, passive, or toggle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SkillType {
    Active,
    Passive,
    Toggle,
}

/// Skill speed determines when a skill executes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SkillSpeed {
    Instant,
    Normal,
    CastTime { seconds: f32 },
}

impl Default for SkillSpeed {
    fn default() -> Self {
        SkillSpeed::Normal
    }
}

/// Effect applied by a skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SkillEffect {
    Damage { base: i32, scaling_stat: String, multiplier: f32 },
    Heal { base: i32, scaling_stat: String, multiplier: f32 },
    ApplyStatus { status: String, duration: i32 },
    FreezeAtb { duration: f32 },
    RemoveStatus { status: String },
    ModifyStat { stat: String, amount: i32, duration: i32 },
}

/// Whether a skill is class-exclusive or learnable by anyone.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SkillExclusivity {
    ClassOnly,
    LearnableByAll,
}

impl Default for SkillExclusivity {
    fn default() -> Self {
        SkillExclusivity::LearnableByAll
    }
}

/// A combat skill definition loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSkillDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub class_origin: String,
    #[serde(rename = "type")]
    pub skill_type: SkillType,
    #[serde(default)]
    pub exclusivity: SkillExclusivity,
    pub resource_cost: Option<i32>,
    #[serde(default)]
    pub cooldown_turns: i32,
    #[serde(default)]
    pub speed: SkillSpeed,
    #[serde(default)]
    pub can_be_interrupted: bool,
    pub target: TargetType,
    pub effects: Vec<SkillEffect>,
    pub stat_requirements: Option<StatBlock>,
    pub icon: String,
}

/// Container for all combat skill definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSkillsData {
    pub combat_skills: Vec<CombatSkillDef>,
}
