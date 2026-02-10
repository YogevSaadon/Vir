//! Asset system — AssetRef with placeholder fallback

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
