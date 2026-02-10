//! Story engine — processes story JSON: current node, available choices, transitions

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::story::choice::{StoryChoice, Rewards};

/// A single node in a story graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryNode {
    pub text: String,
    pub speaker: Option<String>,
    pub background: Option<String>,
    #[serde(default)]
    pub choices: Vec<StoryChoice>,
    pub rewards: Option<Rewards>,
    #[serde(rename = "type")]
    pub node_type: Option<String>,
    #[serde(default)]
    pub tags_to_set: Vec<String>,
    #[serde(default)]
    pub unlock_stories: Vec<String>,
}

/// A complete story definition loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryDef {
    pub story_id: String,
    pub title: String,
    pub entry_node: String,
    #[serde(default)]
    pub required_tags: Vec<String>,
    pub nodes: HashMap<String, StoryNode>,
}

/// Runtime state for an active story.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryState {
    pub story_id: String,
    pub current_node: String,
    pub is_complete: bool,
}

impl StoryState {
    /// Creates a new story state from a definition.
    pub fn new(story: &StoryDef) -> Self {
        Self {
            story_id: story.story_id.clone(),
            current_node: story.entry_node.clone(),
            is_complete: false,
        }
    }

    /// Advances to the next node. Returns the new node ID.
    pub fn advance(&mut self, next_node: &str) {
        self.current_node = next_node.to_string();
    }

    /// Marks the story as complete.
    pub fn complete(&mut self) {
        self.is_complete = true;
    }
}

/// Gets the current node from a story definition and state.
pub fn get_current_node<'a>(
    story: &'a StoryDef,
    state: &StoryState,
) -> Option<&'a StoryNode> {
    story.nodes.get(&state.current_node)
}

/// Checks if a story node is an ending node.
pub fn is_story_end(node: &StoryNode) -> bool {
    node.node_type.as_deref() == Some("story_end")
}
