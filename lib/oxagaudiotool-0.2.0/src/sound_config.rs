use crate::error::error::OxAgAudioToolError;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::sound::{EndPosition, PlaybackPosition, Region};
use kira::tween::Value;
use kira::Volume;

/// Configures any sound that should be played via this library
/// # Properties
/// - path - the path to the audio file, supported formats are: **mp3**, **ogg**, **wav**, **flac**
/// - settings - the actual [StaticSoundSettings]
#[derive(Debug)]
pub struct OxAgSoundConfig {
    pub(crate) path: String,
    pub(crate) settings: StaticSoundSettings,
}

impl OxAgSoundConfig {
    /// # Params
    /// - path - the path to the audio file, supported formats are: **mp3**, **ogg**, **wav**, **flac**
    pub fn new(path: &str) -> OxAgSoundConfig {
        OxAgSoundConfig {
            path: path.to_string(),
            settings: StaticSoundSettings::default(),
        }
    }

    /// # Params
    /// - path - the path to the audio file, supported formats are: **mp3**, **ogg**, **wav**, **flac**
    /// - settings - fully customisable sound settings from the kira library
    pub fn new_with_settings(path: &str, settings: StaticSoundSettings) -> OxAgSoundConfig {
        OxAgSoundConfig { path: path.to_string(), settings }
    }

    /// # Params
    /// - path - the path to the audio file, supported formats are: **mp3**, **ogg**, **wav**, **flac**
    ///
    /// # Side effects
    /// When the sound gets played it will get looped infinitely by default
    pub fn new_looped(path: &str) -> OxAgSoundConfig {
        let settings = StaticSoundSettings::new()
            .loop_region(Region {
                start: PlaybackPosition::Samples(0),
                end: EndPosition::EndOfAudio,
            });

        OxAgSoundConfig { path: path.to_string(), settings }
    }

    /// # Params
    /// - path - the path to the audio file, supported formats are: **mp3**, **ogg**, **wav**, **flac**
    /// - amplitude - the sound will be played with the default volume multiplied by the provided amplitude
    pub fn new_with_volume(path: &str, amplitude: f64) -> OxAgSoundConfig {
        let settings = StaticSoundSettings::new()
            .volume(Value::Fixed(Volume::Amplitude(amplitude)));

        OxAgSoundConfig { path: path.to_string(), settings }
    }

    /// # Params
    /// - path - the path to the audio file, supported formats are: **mp3**, **ogg**, **wav**, **flac**
    /// - amplitude - the sound will be played with the default volume multiplied by the provided amplitude
    ///
    /// # Side effects
    /// When the sound gets played it will get looped infinitely by default
    pub fn new_looped_with_volume(path: &str, amplitude: f64) -> OxAgSoundConfig {
        let settings = StaticSoundSettings::new()
            .loop_region(Region {
                start: PlaybackPosition::Samples(0),
                end: EndPosition::EndOfAudio,
            })
            .volume(Value::Fixed(Volume::Amplitude(amplitude)));

        OxAgSoundConfig { path: path.to_string(), settings }
    }

    /// When the sound gets played it will get looped infinitely by default
    pub fn looped(&mut self) {
        self.settings = self.settings.loop_region(Region {
            start: PlaybackPosition::Samples(0),
            end: EndPosition::EndOfAudio,
        });
    }

    /// The sound will be played with the default volume multiplied by the provided amplitude
    pub fn volume(&mut self, amplitude: f64) {
        self.settings = self.settings.volume(Value::Fixed(Volume::Amplitude(amplitude)));
    }

    pub(crate) fn to_sound_data(&self) -> Result<StaticSoundData, OxAgAudioToolError> {
        Ok(StaticSoundData::from_file(&self.path, self.settings)?)
    }
}