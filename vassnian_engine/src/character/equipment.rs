//! Equipment system — slots, stat modifiers from gear

use serde::{Deserialize, Serialize};
use crate::character::stats::StatBlock;

/// Equipment slot types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EquipSlot {
    Helmet,
    Necklace,
    Armor,
    Boots,
    MainHand,
    OffHand,
    Ring,
    /// Legacy — maps to MainHand
    Weapon,
    /// Legacy — maps to Ring
    Accessory,
}

impl EquipSlot {
    /// Returns the display name for this slot.
    pub fn display_name(&self) -> &'static str {
        match self {
            EquipSlot::Helmet => "Helmet",
            EquipSlot::Necklace => "Necklace",
            EquipSlot::Armor => "Armor",
            EquipSlot::Boots => "Boots",
            EquipSlot::MainHand | EquipSlot::Weapon => "Main Hand",
            EquipSlot::OffHand => "Off Hand",
            EquipSlot::Ring | EquipSlot::Accessory => "Ring",
        }
    }

    /// Normalizes legacy slot names to new ones.
    pub fn normalized(&self) -> EquipSlot {
        match self {
            EquipSlot::Weapon => EquipSlot::MainHand,
            EquipSlot::Accessory => EquipSlot::Ring,
            other => other.clone(),
        }
    }
}

/// Weapon handedness — determines if off-hand is locked.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WeaponHand {
    OneHanded,
    TwoHanded,
    OffHandOnly,
}

impl Default for WeaponHand {
    fn default() -> Self {
        WeaponHand::OneHanded
    }
}

/// A piece of equipped gear with stat bonuses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquippedItem {
    pub item_id: String,
    pub name: String,
    pub slot: EquipSlot,
    pub stat_bonuses: StatBlock,
    #[serde(default)]
    pub weapon_hand: Option<WeaponHand>,
    #[serde(default)]
    pub armor: i32,
    #[serde(default)]
    pub magic_resist: i32,
    #[serde(default)]
    pub shield_block: i32,
}

/// All equipment slots for a character.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EquipmentSlots {
    #[serde(default)]
    pub helmet: Option<EquippedItem>,
    #[serde(default)]
    pub necklace: Option<EquippedItem>,
    #[serde(default)]
    pub armor: Option<EquippedItem>,
    #[serde(default)]
    pub boots: Option<EquippedItem>,
    #[serde(default)]
    pub main_hand: Option<EquippedItem>,
    #[serde(default)]
    pub off_hand: Option<EquippedItem>,
    #[serde(default)]
    pub ring1: Option<EquippedItem>,
    #[serde(default)]
    pub ring2: Option<EquippedItem>,
    #[serde(default)]
    pub ring3: Option<EquippedItem>,
    // Legacy fields for backwards compat with existing saves
    #[serde(default, skip_serializing)]
    weapon: Option<EquippedItem>,
    #[serde(default, skip_serializing)]
    accessory: Option<EquippedItem>,
}

impl EquipmentSlots {
    /// Migrates legacy weapon/accessory fields to new slots after deserialization.
    pub fn migrate_legacy(&mut self) {
        if let Some(wpn) = self.weapon.take() {
            if self.main_hand.is_none() {
                self.main_hand = Some(wpn);
            }
        }
        if let Some(acc) = self.accessory.take() {
            if self.ring1.is_none() {
                self.ring1 = Some(acc);
            }
        }
    }

    fn all_slots(&self) -> [&Option<EquippedItem>; 9] {
        [
            &self.helmet, &self.necklace, &self.armor, &self.boots,
            &self.main_hand, &self.off_hand,
            &self.ring1, &self.ring2, &self.ring3,
        ]
    }

    /// Returns total stat bonuses from all equipped items.
    pub fn total_bonuses(&self) -> StatBlock {
        let mut total = StatBlock::default();
        for item_opt in self.all_slots() {
            if let Some(equipped) = item_opt {
                total = total.add(&equipped.stat_bonuses);
            }
        }
        total
    }

    /// Returns total armor from all equipped items.
    pub fn total_armor(&self) -> i32 {
        self.all_slots().iter()
            .filter_map(|s| s.as_ref())
            .map(|e| e.armor)
            .sum()
    }

    /// Returns total magic resistance from all equipped items.
    pub fn total_mr(&self) -> i32 {
        self.all_slots().iter()
            .filter_map(|s| s.as_ref())
            .map(|e| e.magic_resist)
            .sum()
    }

    /// Returns total shield block from all equipped items.
    pub fn total_shield(&self) -> i32 {
        self.all_slots().iter()
            .filter_map(|s| s.as_ref())
            .map(|e| e.shield_block)
            .sum()
    }

    /// Returns true if a two-handed weapon is in main hand (locks off-hand).
    pub fn is_off_hand_locked(&self) -> bool {
        self.main_hand.as_ref()
            .and_then(|item| item.weapon_hand.as_ref())
            .map_or(false, |h| *h == WeaponHand::TwoHanded)
    }

    /// Equips an item, returning the previously equipped item if any.
    pub fn equip(&mut self, item: EquippedItem) -> Option<EquippedItem> {
        let slot = item.slot.normalized();
        match slot {
            EquipSlot::Helmet => self.helmet.replace(item),
            EquipSlot::Necklace => self.necklace.replace(item),
            EquipSlot::Armor => self.armor.replace(item),
            EquipSlot::Boots => self.boots.replace(item),
            EquipSlot::MainHand | EquipSlot::Weapon => self.main_hand.replace(item),
            EquipSlot::OffHand => self.off_hand.replace(item),
            EquipSlot::Ring | EquipSlot::Accessory => {
                // Fill first empty ring slot, or replace ring1
                if self.ring1.is_none() {
                    self.ring1.replace(item)
                } else if self.ring2.is_none() {
                    self.ring2.replace(item)
                } else if self.ring3.is_none() {
                    self.ring3.replace(item)
                } else {
                    self.ring1.replace(item)
                }
            }
        }
    }

    /// Equips a ring in a specific ring slot (0, 1, or 2).
    pub fn equip_ring(&mut self, ring_index: usize, item: EquippedItem) -> Option<EquippedItem> {
        match ring_index {
            0 => self.ring1.replace(item),
            1 => self.ring2.replace(item),
            2 => self.ring3.replace(item),
            _ => None,
        }
    }

    /// Unequips an item from a slot, returning it.
    pub fn unequip(&mut self, slot: &EquipSlot) -> Option<EquippedItem> {
        let norm = slot.normalized();
        match norm {
            EquipSlot::Helmet => self.helmet.take(),
            EquipSlot::Necklace => self.necklace.take(),
            EquipSlot::Armor => self.armor.take(),
            EquipSlot::Boots => self.boots.take(),
            EquipSlot::MainHand | EquipSlot::Weapon => self.main_hand.take(),
            EquipSlot::OffHand => self.off_hand.take(),
            EquipSlot::Ring | EquipSlot::Accessory => self.ring1.take(),
        }
    }

    /// Unequips a ring from a specific slot (0, 1, or 2).
    pub fn unequip_ring(&mut self, ring_index: usize) -> Option<EquippedItem> {
        match ring_index {
            0 => self.ring1.take(),
            1 => self.ring2.take(),
            2 => self.ring3.take(),
            _ => None,
        }
    }

    /// Gets the equipped item for a given slot.
    pub fn get(&self, slot: &EquipSlot) -> &Option<EquippedItem> {
        let norm = slot.normalized();
        match norm {
            EquipSlot::Helmet => &self.helmet,
            EquipSlot::Necklace => &self.necklace,
            EquipSlot::Armor => &self.armor,
            EquipSlot::Boots => &self.boots,
            EquipSlot::MainHand | EquipSlot::Weapon => &self.main_hand,
            EquipSlot::OffHand => &self.off_hand,
            EquipSlot::Ring | EquipSlot::Accessory => &self.ring1,
        }
    }

    /// Gets a ring by index (0, 1, or 2).
    pub fn get_ring(&self, ring_index: usize) -> &Option<EquippedItem> {
        match ring_index {
            0 => &self.ring1,
            1 => &self.ring2,
            2 => &self.ring3,
            _ => &self.ring1,
        }
    }
}
