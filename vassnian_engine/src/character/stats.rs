//! Stat system — 7 primary stats + derived stats

use serde::{Deserialize, Serialize};

/// Number of primary stats.
pub const STAT_COUNT: usize = 7;

/// Default base value for all stats at character creation.
pub const STAT_BASE_VALUE: i32 = 1;

/// Free stat points available at character creation.
pub const FREE_STAT_POINTS: i32 = 2;

/// The 7 primary stats used by all characters.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StatBlock {
    pub strength: i32,
    pub vitality: i32,
    pub intelligence: i32,
    pub faith: i32,
    pub speed: i32,
    pub dexterity: i32,
    pub luck: i32,
}

impl StatBlock {
    /// Creates a stat block with all stats set to the given value.
    pub fn all(value: i32) -> Self {
        Self {
            strength: value,
            vitality: value,
            intelligence: value,
            faith: value,
            speed: value,
            dexterity: value,
            luck: value,
        }
    }

    /// Gets a stat value by name (case-insensitive).
    pub fn get(&self, name: &str) -> Option<i32> {
        match name.to_lowercase().as_str() {
            "strength" | "str" => Some(self.strength),
            "vitality" | "vit" => Some(self.vitality),
            "intelligence" | "int" => Some(self.intelligence),
            "faith" | "fai" => Some(self.faith),
            "speed" | "spd" => Some(self.speed),
            "dexterity" | "dex" => Some(self.dexterity),
            "luck" | "lck" => Some(self.luck),
            _ => None,
        }
    }

    /// Sets a stat value by name. Returns true if successful.
    pub fn set(&mut self, name: &str, value: i32) -> bool {
        match name.to_lowercase().as_str() {
            "strength" | "str" => { self.strength = value; true }
            "vitality" | "vit" => { self.vitality = value; true }
            "intelligence" | "int" => { self.intelligence = value; true }
            "faith" | "fai" => { self.faith = value; true }
            "speed" | "spd" => { self.speed = value; true }
            "dexterity" | "dex" => { self.dexterity = value; true }
            "luck" | "lck" => { self.luck = value; true }
            _ => false,
        }
    }

    /// Returns total of all stat points allocated.
    pub fn total(&self) -> i32 {
        self.strength + self.vitality + self.intelligence
            + self.faith + self.speed + self.dexterity + self.luck
    }

    /// Adds another stat block to this one (for equipment bonuses etc).
    pub fn add(&self, other: &StatBlock) -> StatBlock {
        StatBlock {
            strength: self.strength + other.strength,
            vitality: self.vitality + other.vitality,
            intelligence: self.intelligence + other.intelligence,
            faith: self.faith + other.faith,
            speed: self.speed + other.speed,
            dexterity: self.dexterity + other.dexterity,
            luck: self.luck + other.luck,
        }
    }

    /// Returns an iterator over (name, value) pairs.
    pub fn iter(&self) -> Vec<(&'static str, i32)> {
        vec![
            ("Strength", self.strength),
            ("Vitality", self.vitality),
            ("Intelligence", self.intelligence),
            ("Faith", self.faith),
            ("Speed", self.speed),
            ("Dexterity", self.dexterity),
            ("Luck", self.luck),
        ]
    }
}

/// Derived stats computed from primary stats.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DerivedStats {
    pub max_hp: i32,
    pub max_mana: i32,
    pub physical_damage: i32,
    pub magic_damage: i32,
    pub defense: i32,
    pub evasion: f32,
    pub crit_chance: f32,
    pub atb_speed: f32,
}

impl DerivedStats {
    /// Calculates derived stats from a primary stat block.
    pub fn from_stats(stats: &StatBlock) -> Self {
        Self {
            max_hp: 10 + stats.vitality * 5,
            max_mana: 5 + stats.intelligence * 3,
            physical_damage: stats.strength * 2,
            magic_damage: stats.intelligence * 2,
            defense: stats.vitality + stats.strength / 2,
            evasion: (stats.dexterity as f32 * 2.0 + stats.speed as f32) / 100.0,
            crit_chance: (stats.luck as f32 * 2.0 + stats.dexterity as f32) / 100.0,
            atb_speed: 1.0 + stats.speed as f32 * 0.05,
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
pub const STAT_DESCRIPTIONS: [(&str, &str, &str); 7] = [
    ("STR", "Strength", "Physical power. Increases melee damage and some skill effects."),
    ("VIT", "Vitality", "Toughness. Increases HP and defense."),
    ("INT", "Intelligence", "Mental acuity. Increases mana and magic damage."),
    ("FAI", "Faith", "Divine connection. Increases healing power and holy skills."),
    ("SPD", "Speed", "Quickness. Faster ATB fill rate in combat."),
    ("DEX", "Dexterity", "Precision. Increases evasion and crit chance."),
    ("LCK", "Luck", "Fortune. Increases crit chance and rare drops."),
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
        let stats = StatBlock::all(1);
        assert_eq!(stats.total(), 7);
    }

    #[test]
    fn test_stat_get_set() {
        let mut stats = StatBlock::all(1);
        assert_eq!(stats.get("strength"), Some(1));
        stats.set("strength", 5);
        assert_eq!(stats.get("str"), Some(5));
    }

    #[test]
    fn test_derived_stats() {
        let stats = StatBlock::all(1);
        let derived = DerivedStats::from_stats(&stats);
        assert_eq!(derived.max_hp, 15);
        assert_eq!(derived.max_mana, 8);
    }

    #[test]
    fn test_stat_add() {
        let base = StatBlock::all(1);
        let bonus = StatBlock { strength: 2, ..Default::default() };
        let total = base.add(&bonus);
        assert_eq!(total.strength, 3);
        assert_eq!(total.vitality, 1);
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
