//! Game phase progression — tracks story phases and time

use serde::{Deserialize, Serialize};

/// Game phases that structure the narrative arc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum GamePhase {
    Phase0Arrival,
    Phase1Growth,
    Phase1_5ShadowEntry,
    Phase2ShadowWorld,
    Phase2_5Emergence,
    Phase3Revenge,
    Phase4FinalBattle,
    Phase4_5Epilogue,
}

impl Default for GamePhase {
    fn default() -> Self {
        GamePhase::Phase0Arrival
    }
}

impl GamePhase {
    /// Returns a display name for UI.
    pub fn display_name(&self) -> &'static str {
        match self {
            GamePhase::Phase0Arrival => "Arrival",
            GamePhase::Phase1Growth => "Growth",
            GamePhase::Phase1_5ShadowEntry => "Shadow World Entry",
            GamePhase::Phase2ShadowWorld => "The Shadow World",
            GamePhase::Phase2_5Emergence => "Emergence",
            GamePhase::Phase3Revenge => "Revenge & Preparation",
            GamePhase::Phase4FinalBattle => "Final Battle",
            GamePhase::Phase4_5Epilogue => "Epilogue",
        }
    }

    /// Returns the numeric order index for comparisons.
    pub fn order(&self) -> u8 {
        match self {
            GamePhase::Phase0Arrival => 0,
            GamePhase::Phase1Growth => 1,
            GamePhase::Phase1_5ShadowEntry => 2,
            GamePhase::Phase2ShadowWorld => 3,
            GamePhase::Phase2_5Emergence => 4,
            GamePhase::Phase3Revenge => 5,
            GamePhase::Phase4FinalBattle => 6,
            GamePhase::Phase4_5Epilogue => 7,
        }
    }
}

/// Default phase time thresholds for progression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseThresholds {
    pub phase_0_to_1: i32,
    pub phase_1_to_1_5: i32,
    pub phase_2_to_2_5: i32,
    pub phase_3_to_4: i32,
}

impl Default for PhaseThresholds {
    fn default() -> Self {
        Self {
            phase_0_to_1: 5,
            phase_1_to_1_5: 20,
            phase_2_to_2_5: 30,
            phase_3_to_4: 25,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_default() {
        assert_eq!(GamePhase::default(), GamePhase::Phase0Arrival);
    }

    #[test]
    fn test_phase_ordering() {
        assert!(GamePhase::Phase0Arrival.order() < GamePhase::Phase1Growth.order());
        assert!(GamePhase::Phase3Revenge.order() < GamePhase::Phase4FinalBattle.order());
    }

    #[test]
    fn test_phase_serde_roundtrip() {
        let phase = GamePhase::Phase2ShadowWorld;
        let json = serde_json::to_string(&phase).unwrap();
        let parsed: GamePhase = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, phase);
    }
}
