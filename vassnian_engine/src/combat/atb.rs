//! ATB bar logic — fill rate based on DEX, reset, pause/resume

use serde::{Deserialize, Serialize};

/// Base ATB fill duration in seconds (at DEX=0).
pub const ATB_BASE_TICKS: f32 = 100.0;

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
    /// Creates a new ATB bar with fill rate based on DEX stat.
    /// ATB Speed = 10 + DEX. Higher = faster fill.
    pub fn new(dex_stat: i32) -> Self {
        let atb_speed = (10 + dex_stat) as f32;
        // Fill rate: atb_speed / base_ticks gives fills/second
        // With DEX=3 (ATB=13): fills in ~100/13 = ~7.7 seconds
        // With DEX=6 (ATB=16): fills in ~100/16 = ~6.3 seconds
        // With DEX=13 (ATB=23): fills in ~100/23 = ~4.3 seconds
        Self {
            current: 0.0,
            fill_rate: atb_speed / ATB_BASE_TICKS,
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
        let mut bar = AtbBar::new(3); // ATB speed = 13, fill_rate = 13/100 = 0.13
        assert!(!bar.is_full());
        // After ~8 seconds should be full (13/100 * 8 = 1.04)
        for _ in 0..800 {
            bar.update(0.01);
        }
        assert!(bar.is_full());
    }

    #[test]
    fn test_atb_pause() {
        let mut bar = AtbBar::new(3);
        bar.pause();
        bar.update(100.0);
        assert_eq!(bar.current, 0.0);
        bar.resume();
        bar.update(100.0);
        assert!(bar.is_full());
    }

    #[test]
    fn test_atb_reset() {
        let mut bar = AtbBar::new(3);
        bar.update(100.0);
        assert!(bar.is_full());
        bar.reset();
        assert!(!bar.is_full());
        assert_eq!(bar.current, 0.0);
    }

    #[test]
    fn test_higher_dex_fills_faster() {
        let low = AtbBar::new(3);  // ATB=13
        let high = AtbBar::new(10); // ATB=20
        assert!(high.fill_rate > low.fill_rate);
    }
}
