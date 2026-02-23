//! Party system — Godsent group management, companion tracking, story skip tracking

use serde::{Deserialize, Serialize};
use super::entity::{Entity, EntityId, EntityKind};

/// Maximum party size (player + companions).
pub const MAX_PARTY_SIZE: usize = 4;

/// The player's party — the Godsent group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    pub members: Vec<Entity>,
    pub gold: i32,
    /// Companion story missions that have been permanently missed.
    #[serde(default)]
    pub missed_companion_stories: Vec<String>,
    /// Companion IDs whose personal quest chains are complete.
    #[serde(default)]
    pub completed_companion_arcs: Vec<String>,
}

impl Default for Party {
    fn default() -> Self {
        Self {
            members: Vec::new(),
            gold: 0,
            missed_companion_stories: Vec::new(),
            completed_companion_arcs: Vec::new(),
        }
    }
}

impl Party {
    /// Creates a new party with starting gold.
    pub fn new(starting_gold: i32) -> Self {
        Self {
            gold: starting_gold,
            ..Default::default()
        }
    }

    /// Adds a member to the party. Returns false if party is full.
    pub fn add_member(&mut self, entity: Entity) -> bool {
        if self.members.len() >= MAX_PARTY_SIZE {
            return false;
        }
        self.members.push(entity);
        true
    }

    /// Removes a member by ID. Returns the removed entity if found.
    pub fn remove_member(&mut self, id: EntityId) -> Option<Entity> {
        if let Some(pos) = self.members.iter().position(|m| m.id == id) {
            Some(self.members.remove(pos))
        } else {
            None
        }
    }

    /// Returns the player entity.
    pub fn player(&self) -> Option<&Entity> {
        self.members.iter().find(|m| m.kind == EntityKind::Player)
    }

    /// Returns the player entity mutably.
    pub fn player_mut(&mut self) -> Option<&mut Entity> {
        self.members.iter_mut().find(|m| m.kind == EntityKind::Player)
    }

    /// Returns all companions.
    pub fn companions(&self) -> Vec<&Entity> {
        self.members.iter()
            .filter(|m| m.kind == EntityKind::Companion)
            .collect()
    }

    /// Returns a specific member by ID.
    pub fn get_member(&self, id: EntityId) -> Option<&Entity> {
        self.members.iter().find(|m| m.id == id)
    }

    /// Returns a specific member mutably by ID.
    pub fn get_member_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.members.iter_mut().find(|m| m.id == id)
    }

    /// Returns all living members (HP > 0).
    pub fn living_members(&self) -> Vec<&Entity> {
        self.members.iter().filter(|m| m.is_alive()).collect()
    }

    /// Returns all world skills across all party members.
    pub fn all_world_skills(&self) -> Vec<&String> {
        self.members.iter()
            .flat_map(|m| m.world_skills.iter())
            .collect()
    }

    /// Checks if any party member has a specific world skill.
    pub fn has_world_skill(&self, skill_id: &str) -> bool {
        self.members.iter().any(|m| m.has_world_skill(skill_id))
    }

    /// Records a missed companion story (can never be replayed).
    pub fn miss_companion_story(&mut self, story_id: &str) {
        if !self.missed_companion_stories.contains(&story_id.to_string()) {
            self.missed_companion_stories.push(story_id.to_string());
        }
    }

    /// Records a completed companion arc.
    pub fn complete_companion_arc(&mut self, companion_id: &str) {
        if !self.completed_companion_arcs.contains(&companion_id.to_string()) {
            self.completed_companion_arcs.push(companion_id.to_string());
        }
    }

    /// Returns true if a companion's story was permanently missed.
    pub fn is_story_missed(&self, story_id: &str) -> bool {
        self.missed_companion_stories.iter().any(|s| s == story_id)
    }

    /// Modifies gold. Returns the new amount. Clamps at 0.
    pub fn modify_gold(&mut self, amount: i32) -> i32 {
        self.gold = (self.gold + amount).max(0);
        self.gold
    }

    /// Returns the number of living party members.
    pub fn alive_count(&self) -> usize {
        self.members.iter().filter(|m| m.is_alive()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::stats::StatBlock;

    fn make_player() -> Entity {
        Entity::new(1, "Godsent".to_string(), EntityKind::Player, "knight".to_string(), StatBlock::all(5))
    }

    fn make_companion(id: EntityId, name: &str) -> Entity {
        Entity::new(id, name.to_string(), EntityKind::Companion, "knight".to_string(), StatBlock::all(3))
    }

    #[test]
    fn test_party_basics() {
        let mut party = Party::new(50);
        assert_eq!(party.gold, 50);
        assert!(party.add_member(make_player()));
        assert!(party.add_member(make_companion(2, "Aldric")));
        assert_eq!(party.members.len(), 2);
        assert!(party.player().is_some());
        assert_eq!(party.companions().len(), 1);
    }

    #[test]
    fn test_party_max_size() {
        let mut party = Party::new(0);
        for i in 0..MAX_PARTY_SIZE {
            assert!(party.add_member(make_companion(i as u32 + 1, "C")));
        }
        assert!(!party.add_member(make_companion(99, "Overflow")));
    }

    #[test]
    fn test_companion_story_tracking() {
        let mut party = Party::new(0);
        party.miss_companion_story("aldric_story_1");
        assert!(party.is_story_missed("aldric_story_1"));
        assert!(!party.is_story_missed("other_story"));

        // No duplicates
        party.miss_companion_story("aldric_story_1");
        assert_eq!(party.missed_companion_stories.len(), 1);
    }

    #[test]
    fn test_gold_modification() {
        let mut party = Party::new(50);
        assert_eq!(party.modify_gold(-30), 20);
        assert_eq!(party.modify_gold(-100), 0); // Clamped at 0
        assert_eq!(party.modify_gold(10), 10);
    }

    #[test]
    fn test_world_skills_across_party() {
        let mut party = Party::new(0);
        let mut player = make_player();
        player.world_skills.push("climbing".to_string());
        let mut companion = make_companion(2, "Aldric");
        companion.world_skills.push("persuasion".to_string());
        party.add_member(player);
        party.add_member(companion);

        assert!(party.has_world_skill("climbing"));
        assert!(party.has_world_skill("persuasion"));
        assert!(!party.has_world_skill("lockpicking"));
        assert_eq!(party.all_world_skills().len(), 2);
    }
}
