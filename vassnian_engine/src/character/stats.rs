//! Stat system — 3 primary stats (STR/DEX/INT) + derived stats

use serde::{Deserialize, Serialize};

/// Number of primary stats.
pub const STAT_COUNT: usize = 3;

/// Default base value for all stats at character creation.
pub const STAT_BASE_VALUE: i32 = 3;

/// Free stat points available at character creation.
pub const FREE_STAT_POINTS: i32 = 3;

/// The 3 primary stats used by all characters.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StatBlock {
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
}

impl StatBlock {
    /// Creates a stat block with all stats set to the given value.
    pub fn all(value: i32) -> Self {
        Self {
            strength: value,
            dexterity: value,
            intelligence: value,
        }
    }

    /// Gets a stat value by name (case-insensitive).
    pub fn get(&self, name: &str) -> Option<i32> {
        match name.to_lowercase().as_str() {
            "strength" | "str" => Some(self.strength),
            "dexterity" | "dex" => Some(self.dexterity),
            "intelligence" | "int" => Some(self.intelligence),
            _ => None,
        }
    }

    /// Sets a stat value by name. Returns true if successful.
    pub fn set(&mut self, name: &str, value: i32) -> bool {
        match name.to_lowercase().as_str() {
            "strength" | "str" => { self.strength = value; true }
            "dexterity" | "dex" => { self.dexterity = value; true }
            "intelligence" | "int" => { self.intelligence = value; true }
            _ => false,
        }
    }

    /// Returns total of all stat points allocated.
    pub fn total(&self) -> i32 {
        self.strength + self.dexterity + self.intelligence
    }

    /// Adds another stat block to this one (for equipment bonuses etc).
    pub fn add(&self, other: &StatBlock) -> StatBlock {
        StatBlock {
            strength: self.strength + other.strength,
            dexterity: self.dexterity + other.dexterity,
            intelligence: self.intelligence + other.intelligence,
        }
    }

    /// Returns an iterator over (name, value) pairs.
    pub fn iter(&self) -> Vec<(&'static str, i32)> {
        vec![
            ("Strength", self.strength),
            ("Dexterity", self.dexterity),
            ("Intelligence", self.intelligence),
        ]
    }
}

/// Derived stats computed from primary stats + level.
/// HP = 6 + STR * Level, Mana = INT + Level, ATB = 10 + DEX
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DerivedStats {
    pub max_hp: i32,
    pub max_mana: i32,
    pub atb_speed: i32,
}

impl DerivedStats {
    /// Calculates derived stats from primary stats and character level.
    pub fn from_stats_at_level(stats: &StatBlock, level: i32) -> Self {
        Self {
            max_hp: 6 + stats.strength * level,
            max_mana: stats.intelligence + level,
            atb_speed: 10 + stats.dexterity,
        }
    }
}

/// A scaling component for data-driven derived stat formulas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatScaling {
    pub from_stat: String,
    pub multiplier: f32,
}

/// Data-driven derived stat formula loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedStatFormula {
    pub stat_id: String,
    pub display_name: String,
    #[serde(default)]
    pub base_value: f32,
    pub scaling: Vec<StatScaling>,
}

impl DerivedStatFormula {
    /// Evaluates this formula against a stat block, returning the computed value.
    pub fn evaluate(&self, stats: &StatBlock) -> f32 {
        let mut value = self.base_value;
        for scale in &self.scaling {
            if let Some(stat_val) = stats.get(&scale.from_stat) {
                value += stat_val as f32 * scale.multiplier;
            }
        }
        value
    }
}

/// Container for all derived stat formulas (loaded from JSON).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedStatFormulas {
    pub derived_stats: Vec<DerivedStatFormula>,
}

/// Names and descriptions for each stat (for tooltips).
pub const STAT_DESCRIPTIONS: [(&str, &str, &str); 3] = [
    ("STR", "Strength", "Physical power. Increases HP and melee damage."),
    ("DEX", "Dexterity", "Agility. Increases ATB speed and light weapon damage."),
    ("INT", "Intelligence", "Mental acuity. Increases mana pool and spell damage."),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_block_default() {
        let stats = StatBlock::default();
        assert_eq!(stats.total(), 0);
    }

    #[test]
    fn test_stat_block_all() {
        let stats = StatBlock::all(3);
        assert_eq!(stats.total(), 9);
    }

    #[test]
    fn test_stat_get_set() {
        let mut stats = StatBlock::all(3);
        assert_eq!(stats.get("strength"), Some(3));
        stats.set("strength", 5);
        assert_eq!(stats.get("str"), Some(5));
    }

    #[test]
    fn test_derived_stats() {
        // STR=3, DEX=3, INT=3 at level 1
        let stats = StatBlock::all(3);
        let derived = DerivedStats::from_stats_at_level(&stats, 1);
        assert_eq!(derived.max_hp, 9);   // 6 + 3*1
        assert_eq!(derived.max_mana, 4); // 3 + 1
        assert_eq!(derived.atb_speed, 13); // 10 + 3
    }

    #[test]
    fn test_stat_add() {
        let base = StatBlock::all(3);
        let bonus = StatBlock { strength: 2, ..Default::default() };
        let total = base.add(&bonus);
        assert_eq!(total.strength, 5);
        assert_eq!(total.dexterity, 3);
    }

    #[test]
    fn test_derived_stat_formula_evaluate() {
        let formula = DerivedStatFormula {
            stat_id: "physical_power".to_string(),
            display_name: "Physical Power".to_string(),
            base_value: 0.0,
            scaling: vec![
                StatScaling { from_stat: "strength".to_string(), multiplier: 1.5 },
                StatScaling { from_stat: "dexterity".to_string(), multiplier: 0.5 },
            ],
        };
        let stats = StatBlock { strength: 10, dexterity: 4, ..Default::default() };
        let result = formula.evaluate(&stats);
        assert!((result - 17.0).abs() < 0.01); // 10*1.5 + 4*0.5 = 17
    }

    #[test]
    fn test_derived_stat_formula_serde() {
        let json = r#"{
            "stat_id": "spell_power",
            "display_name": "Spell Power",
            "base_value": 0,
            "scaling": [{"from_stat": "intelligence", "multiplier": 1.5}]
        }"#;
        let formula: DerivedStatFormula = serde_json::from_str(json).unwrap();
        assert_eq!(formula.stat_id, "spell_power");
        assert_eq!(formula.scaling.len(), 1);
    }
}
