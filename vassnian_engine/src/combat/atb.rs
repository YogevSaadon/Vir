//! ATB bar logic — fill rate, reset, pause/resume

use serde::{Deserialize, Serialize};

/// Base ATB fill duration in seconds.
pub const ATB_BASE_DURATION: f32 = 4.0;

/// Speed stat modifier per point.
pub const ATB_SPEED_MODIFIER: f32 = 0.05;

/// ATB (Active Time Battle) bar for a single unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtbBar {
    /// Current fill amount (0.0 to 1.0).
    pub current: f32,
    /// Fill rate per second (higher = faster).
    pub fill_rate: f32,
    /// Whether this bar is currently paused.
    pub paused: bool,
}

impl AtbBar {
    /// Creates a new ATB bar with fill rate based on speed stat.
    pub fn new(speed_stat: i32) -> Self {
        let speed_modifier = 1.0 + speed_stat as f32 * ATB_SPEED_MODIFIER;
        Self {
            current: 0.0,
            fill_rate: speed_modifier / ATB_BASE_DURATION,
            paused: false,
        }
    }

    /// Updates the ATB bar by delta time. Returns true if bar just filled.
    pub fn update(&mut self, dt: f32) -> bool {
        if self.paused || self.is_full() {
            return false;
        }
        self.current += self.fill_rate * dt;
        if self.current >= 1.0 {
            self.current = 1.0;
            true
        } else {
            false
        }
    }

    /// Returns true if the bar is completely filled.
    pub fn is_full(&self) -> bool {
        self.current >= 1.0
    }

    /// Resets the bar to empty.
    pub fn reset(&mut self) {
        self.current = 0.0;
    }

    /// Pauses the bar.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resumes the bar.
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Returns fill percentage (0.0 to 1.0).
    pub fn percentage(&self) -> f32 {
        self.current.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atb_fill() {
        let mut bar = AtbBar::new(1);
        assert!(!bar.is_full());
        // With speed 1: fill_rate = 1.05 / 4.0 = 0.2625
        // After ~4 seconds should be full
        for _ in 0..400 {
            bar.update(0.01);
        }
        assert!(bar.is_full());
    }

    #[test]
    fn test_atb_pause() {
        let mut bar = AtbBar::new(1);
        bar.pause();
        bar.update(10.0);
        assert_eq!(bar.current, 0.0);
        bar.resume();
        bar.update(10.0);
        assert!(bar.is_full());
    }

    #[test]
    fn test_atb_reset() {
        let mut bar = AtbBar::new(1);
        bar.update(10.0);
        assert!(bar.is_full());
        bar.reset();
        assert!(!bar.is_full());
        assert_eq!(bar.current, 0.0);
    }
}
