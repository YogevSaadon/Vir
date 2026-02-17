//! Loot system — weighted random loot tables with level filtering

use serde::{Deserialize, Serialize};
use crate::inventory::items::ItemRarity;

/// A loot table definition loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTable {
    pub id: String,
    pub entries: Vec<LootEntry>,
    /// Base gold always given.
    #[serde(default)]
    pub guaranteed_gold: i32,
    /// Extra gold per mission level.
    #[serde(default)]
    pub gold_per_level: i32,
    /// How many times to roll on the table.
    #[serde(default = "default_roll_count")]
    pub roll_count: i32,
}

fn default_roll_count() -> i32 { 1 }

/// A single entry in a loot table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootEntry {
    pub item_id: String,
    pub item_type: String,
    /// Higher weight = more common.
    pub weight: i32,
    /// Only drops if mission_level >= this.
    #[serde(default = "default_min_level")]
    pub min_level: i32,
    /// Only drops if mission_level <= this (0 = no cap).
    #[serde(default)]
    pub max_level: i32,
    #[serde(default)]
    pub rarity: ItemRarity,
}

fn default_min_level() -> i32 { 1 }

/// Container for loot tables JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTablesData {
    pub tables: std::collections::HashMap<String, LootTableDef>,
}

/// Raw loot table def from JSON (without id field — id comes from map key).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTableDef {
    pub entries: Vec<LootEntry>,
    #[serde(default)]
    pub guaranteed_gold: i32,
    #[serde(default)]
    pub gold_per_level: i32,
    #[serde(default = "default_roll_count")]
    pub roll_count: i32,
}

impl LootTableDef {
    /// Converts to a LootTable with the given id.
    pub fn with_id(self, id: String) -> LootTable {
        LootTable {
            id,
            entries: self.entries,
            guaranteed_gold: self.guaranteed_gold,
            gold_per_level: self.gold_per_level,
            roll_count: self.roll_count,
        }
    }
}

/// Result of rolling a loot table.
#[derive(Debug, Clone)]
pub struct LootResult {
    pub gold: i32,
    pub items: Vec<String>,
}

/// Rolls on a loot table using a simple RNG seed.
/// `mission_level` filters entries. `luck_bonus` adds % chance of extra rolls.
pub fn roll_loot(
    table: &LootTable,
    mission_level: i32,
    luck_bonus: f32,
    rng_seed: u64,
) -> LootResult {
    let gold = table.guaranteed_gold + table.gold_per_level * mission_level;

    // Filter entries by level
    let eligible: Vec<&LootEntry> = table.entries.iter().filter(|e| {
        mission_level >= e.min_level && (e.max_level == 0 || mission_level <= e.max_level)
    }).collect();

    if eligible.is_empty() {
        return LootResult { gold, items: Vec::new() };
    }

    let total_weight: i32 = eligible.iter().map(|e| e.weight).sum();
    if total_weight <= 0 {
        return LootResult { gold, items: Vec::new() };
    }

    let mut items = Vec::new();
    let mut seed = rng_seed;

    // Bonus roll from luck
    let extra_rolls = if luck_bonus > 0.0 {
        let luck_roll = simple_rng(&mut seed) % 100;
        if luck_roll < (luck_bonus * 100.0) as u64 { 1 } else { 0 }
    } else {
        0
    };

    let rolls = table.roll_count + extra_rolls;

    for _ in 0..rolls {
        let roll = (simple_rng(&mut seed) % total_weight as u64) as i32;
        let mut cumulative = 0;
        for entry in &eligible {
            cumulative += entry.weight;
            if roll < cumulative {
                items.push(entry.item_id.clone());
                break;
            }
        }
    }

    LootResult { gold, items }
}

/// Simple xorshift64 RNG — no external dependency needed.
fn simple_rng(seed: &mut u64) -> u64 {
    let mut s = *seed;
    s ^= s << 13;
    s ^= s >> 7;
    s ^= s << 17;
    *seed = s;
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_table() -> LootTable {
        LootTable {
            id: "test".to_string(),
            entries: vec![
                LootEntry {
                    item_id: "hp_potion".to_string(),
                    item_type: "potion".to_string(),
                    weight: 50,
                    min_level: 1,
                    max_level: 0,
                    rarity: ItemRarity::Common,
                },
                LootEntry {
                    item_id: "rare_sword".to_string(),
                    item_type: "equipment".to_string(),
                    weight: 10,
                    min_level: 3,
                    max_level: 5,
                    rarity: ItemRarity::Rare,
                },
            ],
            guaranteed_gold: 5,
            gold_per_level: 3,
            roll_count: 1,
        }
    }

    #[test]
    fn test_gold_calculation() {
        let table = test_table();
        let result = roll_loot(&table, 2, 0.0, 12345);
        assert_eq!(result.gold, 5 + 3 * 2); // 11
    }

    #[test]
    fn test_level_filtering() {
        let table = test_table();
        // At level 1, rare_sword (min_level 3) should not drop
        let result = roll_loot(&table, 1, 0.0, 12345);
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0], "hp_potion");
    }

    #[test]
    fn test_level_filtering_includes_high() {
        let table = test_table();
        // At level 3, both items are eligible
        let result = roll_loot(&table, 3, 0.0, 12345);
        assert_eq!(result.items.len(), 1);
        // Could be either — just check we got something
        assert!(!result.items[0].is_empty());
    }

    #[test]
    fn test_empty_eligible() {
        let table = LootTable {
            id: "empty".to_string(),
            entries: vec![
                LootEntry {
                    item_id: "high_level".to_string(),
                    item_type: "equipment".to_string(),
                    weight: 10,
                    min_level: 10,
                    max_level: 0,
                    rarity: ItemRarity::Rare,
                },
            ],
            guaranteed_gold: 5,
            gold_per_level: 0,
            roll_count: 1,
        };
        let result = roll_loot(&table, 1, 0.0, 12345);
        assert!(result.items.is_empty());
        assert_eq!(result.gold, 5);
    }

    #[test]
    fn test_serde_roundtrip() {
        let json = r#"{
            "tables": {
                "combat_common": {
                    "entries": [
                        {"item_id": "hp_potion", "item_type": "potion", "weight": 50, "min_level": 1, "max_level": 0}
                    ],
                    "guaranteed_gold": 5,
                    "gold_per_level": 3,
                    "roll_count": 1
                }
            }
        }"#;
        let data: LootTablesData = serde_json::from_str(json).unwrap();
        assert!(data.tables.contains_key("combat_common"));
        let table = data.tables.get("combat_common").unwrap();
        assert_eq!(table.entries.len(), 1);
        assert_eq!(table.guaranteed_gold, 5);
    }
}
