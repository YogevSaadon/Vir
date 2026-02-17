//! Story pool — manages available and scheduled stories for randomization

use serde::{Deserialize, Serialize};
use super::mission::MissionOption;

/// Pre-generated story list for a playthrough.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoryPool {
    pub fixed_stories: Vec<String>,
    pub random_pool: Vec<String>,
    pub active_continuations: Vec<String>,
    pub available_missions: Vec<MissionOption>,
}

impl StoryPool {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a story ID to the fixed list (mandatory events).
    pub fn add_fixed(&mut self, story_id: String) {
        if !self.fixed_stories.contains(&story_id) {
            self.fixed_stories.push(story_id);
        }
    }

    /// Adds a story ID to the random pool.
    pub fn add_random(&mut self, story_id: String) {
        if !self.random_pool.contains(&story_id) {
            self.random_pool.push(story_id);
        }
    }

    /// Promotes story continuations after completing a multi-part story.
    pub fn promote_continuations(&mut self, continuation_ids: Vec<String>) {
        for id in continuation_ids {
            self.random_pool.retain(|s| s != &id);
            if !self.active_continuations.contains(&id) {
                self.active_continuations.push(id);
            }
        }
    }

    /// Removes a story from all pools (after completion or permanent skip).
    pub fn remove_story(&mut self, story_id: &str) {
        self.fixed_stories.retain(|s| s != story_id);
        self.random_pool.retain(|s| s != story_id);
        self.active_continuations.retain(|s| s != story_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_story_pool_new() {
        let pool = StoryPool::new();
        assert!(pool.fixed_stories.is_empty());
        assert!(pool.random_pool.is_empty());
        assert!(pool.available_missions.is_empty());
    }

    #[test]
    fn test_story_pool_add_and_remove() {
        let mut pool = StoryPool::new();
        pool.add_fixed("story_king".to_string());
        pool.add_random("story_forest".to_string());
        assert_eq!(pool.fixed_stories.len(), 1);
        assert_eq!(pool.random_pool.len(), 1);

        pool.remove_story("story_forest");
        assert!(pool.random_pool.is_empty());
    }

    #[test]
    fn test_no_duplicate_adds() {
        let mut pool = StoryPool::new();
        pool.add_fixed("s1".to_string());
        pool.add_fixed("s1".to_string());
        assert_eq!(pool.fixed_stories.len(), 1);
    }

    #[test]
    fn test_promote_continuations() {
        let mut pool = StoryPool::new();
        pool.add_random("s2".to_string());
        pool.promote_continuations(vec!["s2".to_string()]);
        assert!(pool.random_pool.is_empty());
        assert_eq!(pool.active_continuations.len(), 1);
    }
}
