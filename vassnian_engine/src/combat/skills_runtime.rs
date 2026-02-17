//! Combat skill runtime — cooldown tracking, cast bar state, status effects

use serde::{Deserialize, Serialize};
use crate::character::entity::EntityId;

/// Tracks cooldown state for a single skill during combat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCooldown {
    pub skill_id: String,
    pub max_turns: i32,
    pub remaining_turns: i32,
}

impl SkillCooldown {
    pub fn new(skill_id: &str, max_turns: i32) -> Self {
        Self {
            skill_id: skill_id.to_string(),
            max_turns,
            remaining_turns: 0,
        }
    }

    /// Starts the cooldown (call after skill use).
    pub fn trigger(&mut self) {
        self.remaining_turns = self.max_turns;
    }

    /// Ticks one turn off the cooldown. Returns true if now ready.
    pub fn tick(&mut self) -> bool {
        if self.remaining_turns > 0 {
            self.remaining_turns -= 1;
        }
        self.remaining_turns == 0
    }

    /// Returns true if the skill is available (not on cooldown).
    pub fn is_ready(&self) -> bool {
        self.remaining_turns == 0
    }
}

/// State for a skill that is currently being cast (CastTime speed).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastState {
    pub skill_id: String,
    pub caster_id: EntityId,
    pub target_id: EntityId,
    pub total_time: f32,
    pub elapsed: f32,
}

impl CastState {
    pub fn new(skill_id: &str, caster_id: EntityId, target_id: EntityId, cast_time: f32) -> Self {
        Self {
            skill_id: skill_id.to_string(),
            caster_id,
            target_id,
            total_time: cast_time,
            elapsed: 0.0,
        }
    }

    /// Advances the cast bar. Returns true if cast is complete.
    pub fn advance(&mut self, dt: f32) -> bool {
        self.elapsed += dt;
        self.elapsed >= self.total_time
    }

    /// Returns cast progress as 0.0 to 1.0.
    pub fn progress(&self) -> f32 {
        if self.total_time <= 0.0 { return 1.0; }
        (self.elapsed / self.total_time).clamp(0.0, 1.0)
    }

    /// Returns true if the cast was interrupted (cancels the cast).
    pub fn interrupt(&self) -> bool {
        // Interrupt check — caller decides if interrupt happens
        true
    }
}

/// A status effect applied to a combat unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub source_id: EntityId,
    pub remaining_turns: i32,
    pub value: f32,
}

/// Types of status effects.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StatusEffectType {
    Stun,
    Poison,
    Burn,
    Slow,
    Bleed,
    StatBuff { stat: String },
    StatDebuff { stat: String },
    Shield,
    Regeneration,
}

impl StatusEffect {
    pub fn new(effect_type: StatusEffectType, source_id: EntityId, duration: i32, value: f32) -> Self {
        Self {
            effect_type,
            source_id,
            remaining_turns: duration,
            value,
        }
    }

    /// Ticks one turn. Returns true if the effect has expired.
    pub fn tick(&mut self) -> bool {
        if self.remaining_turns > 0 {
            self.remaining_turns -= 1;
        }
        self.remaining_turns == 0
    }

    /// Returns true if this effect prevents the unit from acting.
    pub fn prevents_action(&self) -> bool {
        matches!(self.effect_type, StatusEffectType::Stun)
    }
}

/// Equipped skill slots (4-slot bar for combat).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillSlots {
    pub slots: Vec<Option<String>>,
}

impl SkillSlots {
    pub fn new(max_slots: usize) -> Self {
        Self {
            slots: vec![None; max_slots],
        }
    }

    /// Equips a skill to a slot. Returns the previous skill if any.
    pub fn set_slot(&mut self, index: usize, skill_id: Option<String>) -> Option<String> {
        if index < self.slots.len() {
            std::mem::replace(&mut self.slots[index], skill_id)
        } else {
            None
        }
    }

    /// Returns the skill ID at a slot index.
    pub fn get_slot(&self, index: usize) -> Option<&String> {
        self.slots.get(index).and_then(|s| s.as_ref())
    }

    /// Returns all equipped skill IDs.
    pub fn equipped_skills(&self) -> Vec<&String> {
        self.slots.iter().filter_map(|s| s.as_ref()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_cooldown() {
        let mut cd = SkillCooldown::new("fireball", 3);
        assert!(cd.is_ready());
        cd.trigger();
        assert!(!cd.is_ready());
        assert_eq!(cd.remaining_turns, 3);
        cd.tick();
        assert_eq!(cd.remaining_turns, 2);
        cd.tick();
        cd.tick();
        assert!(cd.is_ready());
    }

    #[test]
    fn test_cast_state() {
        let mut cast = CastState::new("heal", 1, 2, 2.0);
        assert!(!cast.advance(0.5));
        assert!((cast.progress() - 0.25).abs() < 0.01);
        assert!(!cast.advance(1.0));
        assert!(cast.advance(0.6));
        assert!((cast.progress() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_status_effect_tick() {
        let mut effect = StatusEffect::new(StatusEffectType::Poison, 1, 3, 5.0);
        assert!(!effect.tick());
        assert!(!effect.tick());
        assert!(effect.tick());
    }

    #[test]
    fn test_stun_prevents_action() {
        let stun = StatusEffect::new(StatusEffectType::Stun, 1, 2, 0.0);
        assert!(stun.prevents_action());
        let poison = StatusEffect::new(StatusEffectType::Poison, 1, 2, 5.0);
        assert!(!poison.prevents_action());
    }

    #[test]
    fn test_skill_slots() {
        let mut slots = SkillSlots::new(4);
        assert_eq!(slots.equipped_skills().len(), 0);
        slots.set_slot(0, Some("fireball".to_string()));
        slots.set_slot(2, Some("heal".to_string()));
        assert_eq!(slots.equipped_skills().len(), 2);
        assert_eq!(slots.get_slot(0), Some(&"fireball".to_string()));
        assert_eq!(slots.get_slot(1), None);
    }
}
