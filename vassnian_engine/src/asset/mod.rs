//! Asset system — AssetRef with placeholder fallback + ImageManager stubs

use std::collections::HashSet;
use serde::{Deserialize, Serialize};

/// Reference to an image asset. If the file is missing, a placeholder is drawn.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetRef {
    /// Path relative to assets/ directory.
    pub path: String,
}

impl AssetRef {
    /// Creates a new asset reference.
    pub fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }

    /// Returns the filename without extension (for placeholder label).
    pub fn label(&self) -> &str {
        self.path.rsplit('/').next()
            .and_then(|f| f.rsplit('.').last())
            .unwrap_or(&self.path)
    }

    /// Returns the full filesystem path given a base directory.
    pub fn full_path(&self, base_dir: &str) -> String {
        format!("{}/{}", base_dir, self.path)
    }
}

/// Manages image loading, caching, and release.
/// MVP: stub implementation that tracks paths but doesn't load real textures.
/// Real implementation will use Macroquad's Texture2D in the game layer.
#[derive(Debug, Clone, Default)]
pub struct ImageManager {
    loaded_paths: HashSet<String>,
}

impl ImageManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks paths as preloaded. Real impl will async-load textures.
    pub fn preload(&mut self, paths: &[String]) {
        for path in paths {
            self.loaded_paths.insert(path.clone());
        }
    }

    /// Releases specified images from memory.
    pub fn release(&mut self, paths: &[String]) {
        for path in paths {
            self.loaded_paths.remove(path);
        }
    }

    /// Releases all images except the ones in the keep list.
    pub fn release_all_except(&mut self, keep: &[String]) {
        let keep_set: HashSet<&String> = keep.iter().collect();
        self.loaded_paths.retain(|p| keep_set.contains(p));
    }

    /// Returns true if an image is loaded.
    pub fn is_loaded(&self, path: &str) -> bool {
        self.loaded_paths.contains(path)
    }

    /// Returns the number of loaded images.
    pub fn loaded_count(&self) -> usize {
        self.loaded_paths.len()
    }
}
