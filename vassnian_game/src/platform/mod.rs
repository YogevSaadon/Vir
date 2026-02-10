//! Platform abstraction — ads stub, save paths, platform detection

use std::path::PathBuf;

/// Returns the save file path for the current platform.
pub fn save_path() -> PathBuf {
    // For desktop: save next to the executable / in current dir
    let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    path.push("save.json");
    path
}

/// Ad service stub — always returns false (no ads in MVP).
pub fn show_rewarded_ad() -> bool {
    false
}

/// Returns true if running on a mobile platform.
pub fn is_mobile() -> bool {
    false
}
