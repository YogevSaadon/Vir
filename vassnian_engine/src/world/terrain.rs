//! Terrain system — affects enemy spawns and class bonuses (Ranger)

use serde::{Deserialize, Serialize};

/// Terrain types for missions and encounters.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Terrain {
    Forest,
    Mountain,
    Desert,
    Swamp,
    Underground,
    City,
    ShadowWorld,
    Plains,
}

impl Default for Terrain {
    fn default() -> Self {
        Terrain::Plains
    }
}

impl Terrain {
    /// Returns a display name for UI.
    pub fn display_name(&self) -> &'static str {
        match self {
            Terrain::Forest => "Forest",
            Terrain::Mountain => "Mountain",
            Terrain::Desert => "Desert",
            Terrain::Swamp => "Swamp",
            Terrain::Underground => "Underground",
            Terrain::City => "City",
            Terrain::ShadowWorld => "Shadow World",
            Terrain::Plains => "Plains",
        }
    }

    /// Returns all terrain variants (for Ranger terrain choice).
    pub fn all() -> &'static [Terrain] {
        &[
            Terrain::Forest,
            Terrain::Mountain,
            Terrain::Desert,
            Terrain::Swamp,
            Terrain::Underground,
            Terrain::City,
            Terrain::ShadowWorld,
            Terrain::Plains,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terrain_default() {
        assert_eq!(Terrain::default(), Terrain::Plains);
    }

    #[test]
    fn test_terrain_display_name() {
        assert_eq!(Terrain::ShadowWorld.display_name(), "Shadow World");
        assert_eq!(Terrain::Forest.display_name(), "Forest");
    }

    #[test]
    fn test_terrain_all_variants() {
        assert_eq!(Terrain::all().len(), 8);
    }

    #[test]
    fn test_terrain_serde_roundtrip() {
        let terrain = Terrain::ShadowWorld;
        let json = serde_json::to_string(&terrain).unwrap();
        assert_eq!(json, "\"shadow_world\"");
        let parsed: Terrain = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, terrain);
    }
}
