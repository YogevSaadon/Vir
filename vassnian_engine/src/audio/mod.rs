//! Audio system — AudioRef with silent fallback, AudioManager trait

pub mod manager;
pub mod refs;

pub use manager::*;
pub use refs::*;
