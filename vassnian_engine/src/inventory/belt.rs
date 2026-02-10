//! Potion belt — max 4 slots, use and destroy

use serde::{Deserialize, Serialize};

/// Maximum potion belt slots.
pub const POTION_BELT_MAX: usize = 4;

/// The potion belt — holds potions for instant use during combat.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PotionBelt {
    /// Each slot holds a potion ID or None if empty.
    pub slots: Vec<Option<String>>,
}

impl PotionBelt {
    /// Creates a new empty belt with max slots.
    pub fn new() -> Self {
        Self {
            slots: vec![None; POTION_BELT_MAX],
        }
    }

    /// Adds a potion to the first empty slot. Returns false if belt is full.
    pub fn add(&mut self, potion_id: &str) -> bool {
        for slot in &mut self.slots {
            if slot.is_none() {
                *slot = Some(potion_id.to_string());
                return true;
            }
        }
        false
    }

    /// Uses a potion at the given slot index. Returns the potion ID if present.
    pub fn use_slot(&mut self, index: usize) -> Option<String> {
        if index < self.slots.len() {
            self.slots[index].take()
        } else {
            None
        }
    }

    /// Removes a potion from a slot back to inventory.
    pub fn remove(&mut self, index: usize) -> Option<String> {
        self.use_slot(index)
    }

    /// Returns the number of filled slots.
    pub fn count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    /// Returns true if the belt is full.
    pub fn is_full(&self) -> bool {
        self.count() >= POTION_BELT_MAX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_belt_add_and_use() {
        let mut belt = PotionBelt::new();
        assert!(belt.add("hp_potion_small"));
        assert_eq!(belt.count(), 1);

        let used = belt.use_slot(0);
        assert_eq!(used, Some("hp_potion_small".to_string()));
        assert_eq!(belt.count(), 0);
    }

    #[test]
    fn test_belt_full() {
        let mut belt = PotionBelt::new();
        for _ in 0..POTION_BELT_MAX {
            assert!(belt.add("hp_potion_small"));
        }
        assert!(belt.is_full());
        assert!(!belt.add("fire_potion"));
    }
}
