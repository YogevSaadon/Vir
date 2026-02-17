//! Story engine — processes story JSON: current node, available choices, transitions

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::story::choice::{StoryChoice, Rewards};

/// Special scripting instructions for story nodes beyond choice→branch.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StoryScript {
    RandomSelect { pool: Vec<String>, count: usize },
    ForcedBattleOutcome { outcome: String },
    TemporaryPartySplit { who_leaves: Vec<String> },
    TimelineJump { node_order: Vec<String> },
}

/// Image description for a story node (used for art generation and placeholders).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryImage {
    pub id: String,
    #[serde(default)]
    pub description: String,
}

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
    #[serde(default)]
    pub scripts: Vec<StoryScript>,
    #[serde(default)]
    pub allows_shop: bool,
    #[serde(default)]
    pub allows_city: bool,
    /// Shop ID to open when allows_shop is true and player clicks SHOP button.
    #[serde(default)]
    pub shop_id: Option<String>,
    /// Per-companion dialogue lines shown alongside main text.
    #[serde(default)]
    pub companion_lines: HashMap<String, String>,
    /// Image reference for this node (art placeholder / generation hint).
    #[serde(default)]
    pub image: Option<StoryImage>,
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
    /// Which act this story belongs to (1, 2, 3, 4). Default 1.
    #[serde(default = "default_act")]
    pub act: i32,
    /// Short summary shown in the journal after completion.
    #[serde(default)]
    pub summary: Option<String>,
    /// Story chain ID — links multi-part stories together (e.g., "aldric_arc").
    #[serde(default)]
    pub story_chain: Option<String>,
    /// Order within a chain (1, 2, 3...). Default 1.
    #[serde(default = "default_chain_order")]
    pub chain_order: i32,
}

fn default_act() -> i32 { 1 }
fn default_chain_order() -> i32 { 1 }

/// A completed story summary entry, stored in the player's journal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorySummaryEntry {
    pub story_id: String,
    pub title: String,
    pub act: i32,
    pub summary: String,
    pub story_chain: Option<String>,
    pub chain_order: i32,
}

/// Journal of completed story summaries, organized by act.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoryJournal {
    pub entries: Vec<StorySummaryEntry>,
}

impl StoryJournal {
    /// Records a completed story into the journal.
    pub fn record(&mut self, story: &StoryDef) {
        // Don't duplicate
        if self.entries.iter().any(|e| e.story_id == story.story_id) {
            return;
        }
        let summary = story.summary.clone()
            .unwrap_or_else(|| format!("Completed: {}", story.title));
        self.entries.push(StorySummaryEntry {
            story_id: story.story_id.clone(),
            title: story.title.clone(),
            act: story.act,
            summary,
            story_chain: story.story_chain.clone(),
            chain_order: story.chain_order,
        });
    }

    /// Returns all entries for a given act, sorted by chain then chain_order.
    pub fn entries_for_act(&self, act: i32) -> Vec<&StorySummaryEntry> {
        let mut entries: Vec<&StorySummaryEntry> = self.entries.iter()
            .filter(|e| e.act == act)
            .collect();
        entries.sort_by(|a, b| {
            a.story_chain.cmp(&b.story_chain)
                .then(a.chain_order.cmp(&b.chain_order))
        });
        entries
    }

    /// Returns all entries for a given story chain across all acts.
    pub fn chain_entries(&self, chain_id: &str) -> Vec<&StorySummaryEntry> {
        let mut entries: Vec<&StorySummaryEntry> = self.entries.iter()
            .filter(|e| e.story_chain.as_deref() == Some(chain_id))
            .collect();
        entries.sort_by_key(|e| (e.act, e.chain_order));
        entries
    }
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
