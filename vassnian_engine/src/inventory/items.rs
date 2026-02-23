//! Item definitions — Potion, Equipment data structs
//! These are the *definitions* loaded from JSON.

use serde::{Deserialize, Serialize};
use crate::character::stats::StatBlock;
use crate::character::entity::AttackType;
use crate::character::equipment::{EquipSlot, WeaponHand};

/// Target type for skills and items.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TargetType {
    #[serde(rename = "self")]
    Self_,
    SingleAlly,
    SingleEnemy,
    AllAllies,
    AllEnemies,
    AllFrontEnemies,
}

/// Potion effect when used.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PotionEffect {
    Heal { amount: i32 },
    Damage { amount: i32, element: String },
}

/// Item rarity for loot tables and shop generation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Unique,
}

impl Default for ItemRarity {
    fn default() -> Self {
        ItemRarity::Common
    }
}

/// High-level item category for the unified item system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ItemCategory {
    Weapon { weapon_type: String },
    Armor { slot: EquipSlot },
    Accessory,
    Potion,
    KeyItem { description: String },
    CraftingMaterial { used_at: String },
    SkillBook { skill_id: String },
}

/// Potion definition loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotionDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub target: TargetType,
    pub effect: PotionEffect,
    pub icon: String,
    pub buy_price: i32,
    pub sell_price: i32,
    #[serde(default = "default_item_level")]
    pub level: i32,
    #[serde(default)]
    pub rarity: ItemRarity,
}

/// Equipment definition loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub slot: EquipSlot,
    pub weapon_type: Option<String>,
    pub attack_type: Option<AttackType>,
    #[serde(default)]
    pub weapon_hand: Option<WeaponHand>,
    pub stat_bonuses: StatBlock,
    pub icon: String,
    pub buy_price: i32,
    pub sell_price: i32,
    #[serde(default = "default_item_level")]
    pub level: i32,
    #[serde(default)]
    pub rarity: ItemRarity,
    #[serde(default)]
    pub armor: i32,
    #[serde(default)]
    pub magic_resist: i32,
    #[serde(default)]
    pub shield_block: i32,
    #[serde(default)]
    pub weapon_base: i32,
    #[serde(default)]
    pub weapon_stat: String,
}

fn default_item_level() -> i32 { 1 }

/// A potion instance in the player's inventory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotionStack {
    pub potion_id: String,
    pub count: i32,
}

/// An equipment instance in the player's inventory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentInstance {
    pub equipment_id: String,
}

/// Shop item entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopItem {
    pub item_id: String,
    pub item_type: String,
    pub stock: i32,
}

/// Shop definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopDef {
    pub name: String,
    pub shopkeeper: String,
    pub items: Vec<ShopItem>,
}
