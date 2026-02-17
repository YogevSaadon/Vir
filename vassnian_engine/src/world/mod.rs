//! World systems — terrain, game phases, missions, story pool, world state

pub mod terrain;
pub mod phase;
pub mod mission;
pub mod story_pool;
pub mod world_state;

pub use terrain::Terrain;
pub use phase::{GamePhase, PhaseThresholds};
pub use mission::{MissionDef, MissionOption, MissionDifficulty, MissionContext};
pub use story_pool::StoryPool;
pub use world_state::WorldState;
