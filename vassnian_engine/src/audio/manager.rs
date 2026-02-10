//! Audio manager — play music (loop), play SFX (one-shot), stop, volume control

use crate::audio::refs::AudioRef;

/// Default music volume.
pub const DEFAULT_MUSIC_VOLUME: f32 = 0.7;

/// Default SFX volume.
pub const DEFAULT_SFX_VOLUME: f32 = 1.0;

/// Audio manager trait — handles music and SFX playback.
pub trait AudioManager {
    /// Plays a music track, optionally looping.
    fn play_music(&mut self, track: &AudioRef, looping: bool);
    /// Stops the current music.
    fn stop_music(&mut self);
    /// Plays a sound effect (one-shot).
    fn play_sfx(&mut self, sfx: &AudioRef);
    /// Sets music volume (0.0 to 1.0).
    fn set_music_volume(&mut self, volume: f32);
    /// Sets SFX volume (0.0 to 1.0).
    fn set_sfx_volume(&mut self, volume: f32);
}

/// Silent audio manager — all methods are no-ops. Used when no audio files exist.
pub struct SilentAudioManager {
    pub music_volume: f32,
    pub sfx_volume: f32,
}

impl SilentAudioManager {
    /// Creates a new silent audio manager with default volumes.
    pub fn new() -> Self {
        Self {
            music_volume: DEFAULT_MUSIC_VOLUME,
            sfx_volume: DEFAULT_SFX_VOLUME,
        }
    }
}

impl Default for SilentAudioManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioManager for SilentAudioManager {
    fn play_music(&mut self, _track: &AudioRef, _looping: bool) {}
    fn stop_music(&mut self) {}
    fn play_sfx(&mut self, _sfx: &AudioRef) {}
    fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
    }
    fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }
}
