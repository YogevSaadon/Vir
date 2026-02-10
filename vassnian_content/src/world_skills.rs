//! World skill definitions

use serde::{Deserialize, Serialize};

/// A world skill definition loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSkillDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
}

/// Container for all world skill definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSkillsData {
    pub world_skills: Vec<WorldSkillDef>,
}
