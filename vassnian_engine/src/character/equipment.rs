//! Equipment system — slots, stat modifiers from gear

use serde::{Deserialize, Serialize};
use crate::character::stats::StatBlock;

/// Equipment slot types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EquipSlot {
    Weapon,
    Armor,
    Accessory,
}

/// A piece of equipped gear with stat bonuses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquippedItem {
    pub item_id: String,
    pub name: String,
    pub slot: EquipSlot,
    pub stat_bonuses: StatBlock,
}

/// All equipment slots for a character.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EquipmentSlots {
    pub weapon: Option<EquippedItem>,
    pub armor: Option<EquippedItem>,
    pub accessory: Option<EquippedItem>,
}

impl EquipmentSlots {
    /// Returns total stat bonuses from all equipped items.
    pub fn total_bonuses(&self) -> StatBlock {
        let mut total = StatBlock::default();
        for item in [&self.weapon, &self.armor, &self.accessory] {
            if let Some(equipped) = item {
                total = total.add(&equipped.stat_bonuses);
            }
        }
        total
    }

    /// Equips an item, returning the previously equipped item if any.
    pub fn equip(&mut self, item: EquippedItem) -> Option<EquippedItem> {
        let slot = match item.slot {
            EquipSlot::Weapon => &mut self.weapon,
            EquipSlot::Armor => &mut self.armor,
            EquipSlot::Accessory => &mut self.accessory,
        };
        slot.replace(item)
    }

    /// Unequips an item from a slot, returning it.
    pub fn unequip(&mut self, slot: &EquipSlot) -> Option<EquippedItem> {
        match slot {
            EquipSlot::Weapon => self.weapon.take(),
            EquipSlot::Armor => self.armor.take(),
            EquipSlot::Accessory => self.accessory.take(),
        }
    }
}
