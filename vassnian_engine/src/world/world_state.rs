//! WorldState — runtime tracking for time, phase progression, mission history

use serde::{Deserialize, Serialize};
use super::phase::{GamePhase, PhaseThresholds};
use super::terrain::Terrain;

/// Runtime world state — persisted in save file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub current_phase: GamePhase,
    pub elapsed_time: i32,
    pub missions_completed: Vec<String>,
    pub missions_failed: Vec<String>,
    pub current_terrain: Terrain,
    #[serde(default)]
    pub phase_thresholds: PhaseThresholds,
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            current_phase: GamePhase::default(),
            elapsed_time: 0,
            missions_completed: Vec::new(),
            missions_failed: Vec::new(),
            current_terrain: Terrain::Plains,
            phase_thresholds: PhaseThresholds::default(),
        }
    }
}

impl WorldState {
    /// Creates a new world state with custom thresholds.
    pub fn with_thresholds(thresholds: PhaseThresholds) -> Self {
        Self {
            phase_thresholds: thresholds,
            ..Default::default()
        }
    }

    /// Advances time by the given amount and checks for phase progression.
    /// Returns the new phase if it changed.
    pub fn advance_time(&mut self, amount: i32) -> Option<GamePhase> {
        self.elapsed_time += amount;
        let old_phase = self.current_phase.clone();
        self.check_phase_progression();
        if self.current_phase != old_phase {
            Some(self.current_phase.clone())
        } else {
            None
        }
    }

    /// Records a mission completion.
    pub fn complete_mission(&mut self, mission_id: &str) {
        if !self.missions_completed.contains(&mission_id.to_string()) {
            self.missions_completed.push(mission_id.to_string());
        }
    }

    /// Records a mission failure (missed companion story, etc.).
    pub fn fail_mission(&mut self, mission_id: &str) {
        if !self.missions_failed.contains(&mission_id.to_string()) {
            self.missions_failed.push(mission_id.to_string());
        }
    }

    /// Returns true if a mission has been completed.
    pub fn is_mission_complete(&self, mission_id: &str) -> bool {
        self.missions_completed.iter().any(|m| m == mission_id)
    }

    /// Sets the current terrain.
    pub fn set_terrain(&mut self, terrain: Terrain) {
        self.current_terrain = terrain;
    }

    /// Checks phase thresholds and advances phase if time warrants it.
    fn check_phase_progression(&mut self) {
        let t = &self.phase_thresholds;
        let time = self.elapsed_time;

        // Phase transitions are one-directional — only forward
        let new_phase = if time >= t.phase_3_to_4 + t.phase_2_to_2_5 + t.phase_1_to_1_5 + t.phase_0_to_1 {
            GamePhase::Phase4FinalBattle
        } else if time >= t.phase_2_to_2_5 + t.phase_1_to_1_5 + t.phase_0_to_1 {
            GamePhase::Phase3Revenge
        } else if time >= t.phase_1_to_1_5 + t.phase_0_to_1 {
            // Phase 2→2.5 handled by story triggers, not time alone
            GamePhase::Phase2ShadowWorld
        } else if time >= t.phase_0_to_1 {
            GamePhase::Phase1Growth
        } else {
            GamePhase::Phase0Arrival
        };

        // Only advance forward, never backward
        if new_phase.order() > self.current_phase.order() {
            self.current_phase = new_phase;
        }
    }

    /// Forces a phase transition (for story-triggered transitions like shadow entry).
    pub fn force_phase(&mut self, phase: GamePhase) {
        if phase.order() >= self.current_phase.order() {
            self.current_phase = phase;
        }
    }

    /// Returns total missions completed count.
    pub fn total_completed(&self) -> usize {
        self.missions_completed.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_state_default() {
        let ws = WorldState::default();
        assert_eq!(ws.current_phase, GamePhase::Phase0Arrival);
        assert_eq!(ws.elapsed_time, 0);
        assert!(ws.missions_completed.is_empty());
    }

    #[test]
    fn test_advance_time_triggers_phase() {
        let mut ws = WorldState::default();
        // Default threshold for phase 0→1 is 5
        let result = ws.advance_time(3);
        assert!(result.is_none());
        assert_eq!(ws.current_phase, GamePhase::Phase0Arrival);

        let result = ws.advance_time(3);
        assert!(result.is_some());
        assert_eq!(ws.current_phase, GamePhase::Phase1Growth);
    }

    #[test]
    fn test_mission_tracking() {
        let mut ws = WorldState::default();
        ws.complete_mission("m_goblin_camp");
        assert!(ws.is_mission_complete("m_goblin_camp"));
        assert!(!ws.is_mission_complete("m_other"));

        // No duplicates
        ws.complete_mission("m_goblin_camp");
        assert_eq!(ws.missions_completed.len(), 1);
    }

    #[test]
    fn test_force_phase_only_forward() {
        let mut ws = WorldState::default();
        ws.force_phase(GamePhase::Phase2ShadowWorld);
        assert_eq!(ws.current_phase, GamePhase::Phase2ShadowWorld);

        // Cannot go backward
        ws.force_phase(GamePhase::Phase0Arrival);
        assert_eq!(ws.current_phase, GamePhase::Phase2ShadowWorld);
    }

    #[test]
    fn test_serde_roundtrip() {
        let mut ws = WorldState::default();
        ws.advance_time(10);
        ws.complete_mission("m_test");
        let json = serde_json::to_string(&ws).unwrap();
        let parsed: WorldState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.elapsed_time, 10);
        assert_eq!(parsed.current_phase, ws.current_phase);
        assert!(parsed.is_mission_complete("m_test"));
    }
}
