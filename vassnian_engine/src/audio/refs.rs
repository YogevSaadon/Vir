//! AudioRef struct — points to audio file, silent if missing

use serde::{Deserialize, Serialize};

/// Reference to an audio file. If missing, plays nothing (silent fallback).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioRef {
    /// Path relative to assets/ directory.
    pub path: String,
}

impl AudioRef {
    /// Creates a new audio reference.
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}
