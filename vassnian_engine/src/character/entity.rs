//! Entity system — Player, Companion, Enemy variants

use serde::{Deserialize, Serialize};
use crate::character::stats::{StatBlock, DerivedStats};
use crate::combat::skills_runtime::SkillSlots;

/// Unique identifier for entities in combat.
pub type EntityId = u32;

/// The kind of entity — determines behavior and AI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EntityKind {
    Player,
    Companion,
    Enemy,
}

/// Formation row for combat positioning.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FormationRow {
    Front,
    Back,
}

/// A game entity — player, companion, or enemy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub kind: EntityKind,
    pub class_id: String,
    pub portrait: String,
    pub stats: StatBlock,
    pub derived: DerivedStats,
    pub current_hp: i32,
    pub current_mana: i32,
    pub combat_skills: Vec<String>,
    pub world_skills: Vec<String>,
    pub position: FormationRow,
    pub attack_type: AttackType,
    pub ai_type: String,
    pub injuries: i32,
    pub level: i32,
    pub exp: i32,
    #[serde(default)]
    pub pending_stat_points: i32,
    #[serde(default)]
    pub skill_slots: SkillSlots,
}

/// Attack type determines targeting rules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AttackType {
    Melee,
    Ranged,
}

impl Entity {
    /// Creates a new entity with full HP/mana from stats.
    pub fn new(
        id: EntityId,
        name: String,
        kind: EntityKind,
        class_id: String,
        stats: StatBlock,
    ) -> Self {
        let derived = DerivedStats::from_stats(&stats);
        let current_hp = derived.max_hp;
        let current_mana = derived.max_mana;
        Self {
            id,
            name,
            kind,
            class_id,
            portrait: String::new(),
            stats,
            derived,
            current_hp,
            current_mana,
            combat_skills: Vec::new(),
            world_skills: Vec::new(),
            position: FormationRow::Front,
            attack_type: AttackType::Melee,
            ai_type: "basic".to_string(),
            injuries: 0,
            level: 1,
            exp: 0,
            pending_stat_points: 0,
            skill_slots: SkillSlots::new(4),
        }
    }

    /// Returns true if this entity is alive (HP > 0).
    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }

    /// Applies damage to this entity. Returns actual damage dealt.
    pub fn take_damage(&mut self, amount: i32) -> i32 {
        let actual = amount.min(self.current_hp);
        self.current_hp -= actual;
        actual
    }

    /// Heals this entity. Returns actual amount healed.
    pub fn heal(&mut self, amount: i32) -> i32 {
        let actual = amount.min(self.derived.max_hp - self.current_hp);
        self.current_hp += actual;
        actual
    }

    /// Recalculates derived stats (call after stat changes).
    pub fn recalculate_derived(&mut self) {
        self.derived = DerivedStats::from_stats(&self.stats);
    }

    /// Returns true if this entity has a specific world skill.
    pub fn has_world_skill(&self, skill_id: &str) -> bool {
        self.world_skills.iter().any(|s| s == skill_id)
    }

    /// Adds an injury. Returns true if permadeath triggered.
    pub fn add_injury(&mut self) -> bool {
        self.injuries += 1;
        self.injuries >= 3
    }
}
