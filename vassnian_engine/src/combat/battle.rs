//! Battle state — units, positions, turn order, potion use

use serde::{Deserialize, Serialize};
use crate::character::entity::{Entity, EntityId, EntityKind};
use crate::combat::atb::AtbBar;
use crate::combat::skills_runtime::{SkillCooldown, CastState, StatusEffect, SkillSlots};

/// Represents a unit in combat with its ATB bar.
#[derive(Debug, Clone)]
pub struct CombatUnit {
    pub entity: Entity,
    pub atb: AtbBar,
    pub is_active: bool,
    pub skill_cooldowns: Vec<SkillCooldown>,
    pub status_effects: Vec<StatusEffect>,
    pub active_cast: Option<CastState>,
    pub skill_slots: SkillSlots,
}

impl CombatUnit {
    /// Creates a combat unit from an entity.
    pub fn new(entity: Entity) -> Self {
        let atb = AtbBar::new(entity.stats.speed);
        Self {
            entity,
            atb,
            is_active: true,
            skill_cooldowns: Vec::new(),
            status_effects: Vec::new(),
            active_cast: None,
            skill_slots: SkillSlots::new(4),
        }
    }

    /// Ticks all cooldowns by one turn.
    pub fn tick_cooldowns(&mut self) {
        for cd in &mut self.skill_cooldowns {
            cd.tick();
        }
    }

    /// Ticks all status effects. Removes expired ones. Returns expired effect types.
    pub fn tick_status_effects(&mut self) -> Vec<StatusEffect> {
        let mut expired = Vec::new();
        self.status_effects.retain_mut(|effect| {
            if effect.tick() {
                expired.push(effect.clone());
                false
            } else {
                true
            }
        });
        expired
    }

    /// Returns true if this unit is stunned (cannot act).
    pub fn is_stunned(&self) -> bool {
        self.status_effects.iter().any(|e| e.prevents_action())
    }

    /// Returns true if a specific skill is ready (off cooldown).
    pub fn is_skill_ready(&self, skill_id: &str) -> bool {
        self.skill_cooldowns.iter()
            .find(|cd| cd.skill_id == skill_id)
            .map(|cd| cd.is_ready())
            .unwrap_or(true)
    }
}

/// The result of a combat encounter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CombatResult {
    InProgress,
    Victory,
    Defeat,
}

/// Actions that can be taken in combat.
#[derive(Debug, Clone)]
pub enum CombatAction {
    /// Basic melee/ranged attack on a target.
    Attack { target_id: EntityId },
    /// Use a combat skill on a target.
    UseSkill { skill_id: String, target_id: EntityId },
    /// Use a potion from belt on a target.
    UsePotion { belt_slot: usize, target_id: EntityId },
    /// Do nothing (wait).
    Wait,
}

/// Full battle state.
#[derive(Debug)]
pub struct BattleState {
    pub allies: Vec<CombatUnit>,
    pub enemies: Vec<CombatUnit>,
    pub paused: bool,
    pub result: CombatResult,
    pub combat_log: Vec<String>,
}

impl BattleState {
    /// Creates a new battle from ally and enemy entities.
    pub fn new(ally_entities: Vec<Entity>, enemy_entities: Vec<Entity>) -> Self {
        let allies = ally_entities.into_iter().map(CombatUnit::new).collect();
        let enemies = enemy_entities.into_iter().map(CombatUnit::new).collect();
        Self {
            allies,
            enemies,
            paused: false,
            result: CombatResult::InProgress,
            combat_log: Vec::new(),
        }
    }

    /// Updates all ATB bars by delta time. Returns IDs of units whose bars just filled.
    pub fn update_atb(&mut self, dt: f32) -> Vec<EntityId> {
        if self.paused || self.result != CombatResult::InProgress {
            return Vec::new();
        }

        let mut ready_units = Vec::new();

        for unit in self.allies.iter_mut().chain(self.enemies.iter_mut()) {
            if unit.is_active && unit.entity.is_alive() && unit.atb.update(dt) {
                ready_units.push(unit.entity.id);
            }
        }

        ready_units
    }

    /// Toggles pause state.
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
        let paused = self.paused;
        for unit in self.allies.iter_mut().chain(self.enemies.iter_mut()) {
            if paused {
                unit.atb.pause();
            } else {
                unit.atb.resume();
            }
        }
    }

    /// Gets a mutable reference to a unit by ID.
    pub fn get_unit_mut(&mut self, id: EntityId) -> Option<&mut CombatUnit> {
        self.allies.iter_mut()
            .chain(self.enemies.iter_mut())
            .find(|u| u.entity.id == id)
    }

    /// Gets an immutable reference to a unit by ID.
    pub fn get_unit(&self, id: EntityId) -> Option<&CombatUnit> {
        self.allies.iter()
            .chain(self.enemies.iter())
            .find(|u| u.entity.id == id)
    }

    /// Returns living ally entities.
    pub fn living_allies(&self) -> Vec<&CombatUnit> {
        self.allies.iter()
            .filter(|u| u.entity.is_alive() && u.is_active)
            .collect()
    }

    /// Returns living enemy entities.
    pub fn living_enemies(&self) -> Vec<&CombatUnit> {
        self.enemies.iter()
            .filter(|u| u.entity.is_alive() && u.is_active)
            .collect()
    }

    /// Checks win/lose conditions and updates result.
    pub fn check_result(&mut self) -> CombatResult {
        if self.living_enemies().is_empty() {
            self.result = CombatResult::Victory;
        } else if self.living_allies().is_empty() {
            self.result = CombatResult::Defeat;
        }
        self.result.clone()
    }

    /// Logs a combat message.
    pub fn log(&mut self, message: String) {
        self.combat_log.push(message);
    }

    /// Returns the player unit (EntityKind::Player).
    pub fn player(&self) -> Option<&CombatUnit> {
        self.allies.iter().find(|u| u.entity.kind == EntityKind::Player)
    }

    /// Returns the player unit mutably.
    pub fn player_mut(&mut self) -> Option<&mut CombatUnit> {
        self.allies.iter_mut().find(|u| u.entity.kind == EntityKind::Player)
    }
}
