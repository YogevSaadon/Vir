//! Tag system — set/get/check tags, tag-based filtering

use std::collections::HashSet;
use serde::{Deserialize, Serialize};

/// Simple tag system — tags are either present or not.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TagSystem {
    tags: HashSet<String>,
}

impl TagSystem {
    /// Creates a new empty tag system.
    pub fn new() -> Self {
        Self { tags: HashSet::new() }
    }

    /// Sets a tag.
    pub fn set(&mut self, tag: &str) {
        self.tags.insert(tag.to_string());
    }

    /// Sets multiple tags at once.
    pub fn set_many(&mut self, tags: &[String]) {
        for tag in tags {
            self.tags.insert(tag.clone());
        }
    }

    /// Checks if a tag is set.
    pub fn has(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }

    /// Checks if all given tags are set.
    pub fn has_all(&self, tags: &[String]) -> bool {
        tags.iter().all(|t| self.tags.contains(t))
    }

    /// Removes a tag.
    pub fn remove(&mut self, tag: &str) {
        self.tags.remove(tag);
    }

    /// Returns all tags as a sorted vector (for serialization).
    pub fn all_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self.tags.iter().cloned().collect();
        tags.sort();
        tags
    }

    /// Loads tags from a vector (for deserialization).
    pub fn from_vec(tags: Vec<String>) -> Self {
        Self {
            tags: tags.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_set_and_check() {
        let mut tags = TagSystem::new();
        tags.set("game_started");
        assert!(tags.has("game_started"));
        assert!(!tags.has("game_ended"));
    }

    #[test]
    fn test_tag_has_all() {
        let mut tags = TagSystem::new();
        tags.set("a");
        tags.set("b");
        assert!(tags.has_all(&["a".to_string(), "b".to_string()]));
        assert!(!tags.has_all(&["a".to_string(), "c".to_string()]));
    }
}
