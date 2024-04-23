pub mod sound_config;
pub mod error;
mod util;
#[cfg(test)]
mod test;

/// Oxidizing Agents Audio Tool
///
/// # Main features
/// - assign each [TileType] its own sound<br>
///   This library will handle the switching between tile types and play the appropriate audio
/// - assign each [WeatherType] its own sound<br>
///   This library will handle the switching between weather types and play the appropriate audio
/// - react to [Event]s with sound
/// - play any sound from a give file with minimal setup
///
/// # Example
/// ```rs
/// use std::collections::HashMap;
/// use std::thread::sleep;
/// use std::time::Duration;
/// use robotics_lib::energy::Energy;
/// use robotics_lib::event::events::Event;
/// use robotics_lib::runner::backpack::BackPack;
/// use robotics_lib::runner::{Robot, Runnable, Runner};
/// use robotics_lib::world::coordinates::Coordinate;
/// use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
/// use robotics_lib::world::tile::{Content, Tile, TileType};
/// use robotics_lib::world::World;
/// use robotics_lib::world::world_generator::Generator;
/// use oxagaudiotool::sound_config::OxAgSoundConfig;
/// use oxagaudiotool::OxAgAudioTool;
/// use oxagaudiotool::error::error::OxAgAudioToolError;
///
/// struct DjRobot {
///     robot: Robot,
///     audio: OxAgAudioTool
/// }
///
/// impl Runnable for DjRobot {
///     fn process_tick(&mut self, _: &mut World) {
///
///     }
///     fn handle_event(&mut self, event: Event) {
///         let _ = self.audio.play_audio_based_on_event(&event);
///
///         println!();
///         println!("{:?}", event);
///         println!();
///     }
///     fn get_energy(&self) -> &Energy {
///         &self.robot.energy
///     }
///     fn get_energy_mut(&mut self) -> &mut Energy {
///         &mut self.robot.energy
///     }
///     fn get_coordinate(&self) -> &Coordinate {
///         &self.robot.coordinate
///     }
///     fn get_coordinate_mut(&mut self) -> &mut Coordinate {
///         &mut self.robot.coordinate
///     }
///     fn get_backpack(&self) -> &BackPack {
///         &self.robot.backpack
///     }
///     fn get_backpack_mut(&mut self) -> &mut BackPack {
///         &mut self.robot.backpack
///     }
/// }
///
/// struct MyGen {}
///
/// impl MyGen {
///     fn new() -> MyGen {
///         MyGen{}
///     }
/// }
///
/// impl Generator for MyGen {
///     fn gen(&mut self) -> robotics_lib::world::world_generator::World {
///         let mut weather = Vec::new();
///         weather.push(WeatherType::Sunny);
///         weather.push(WeatherType::TrentinoSnow);
///         weather.push(WeatherType::TrentinoSnow);
///         weather.push(WeatherType::Rainy);
///         weather.push(WeatherType::Rainy);
///         weather.push(WeatherType::Foggy);
///         weather.push(WeatherType::TropicalMonsoon);
///
///
///         let mut tiles = Vec::new();
///         let mut another = Vec::new();
///         another.push(Tile {
///             tile_type: TileType::DeepWater,
///             content: Content::None,
///             elevation: 0,
///         });
///         tiles.push(another);
///         (tiles, (0, 0), EnvironmentalConditions::new(&weather, 3, 3).unwrap(), 0.1, None)
///     }
/// }
///
/// fn main() -> Result<(), OxAgAudioToolError>{
///     println!("Loading game...");
///
///     let background_music = OxAgSoundConfig::new_looped_with_volume("assets/default/music.ogg", 2.0);
///
///     let mut events = HashMap::new();
///     events.insert(Event::Ready, OxAgSoundConfig::new("assets/default/event/event_ready.ogg"));
///     events.insert(Event::Terminated, OxAgSoundConfig::new("assets/default/event/event_terminated.ogg"));
///     // events.insert(Event::EnergyRecharged(0), OxAgSoundConfig::new_with_volume("assets/default/event/event_energy_recharged.ogg", 0.1));
///     events.insert(Event::AddedToBackpack(Content::None, 0), OxAgSoundConfig::new("assets/default/event/event_add_to_backpack.ogg"));
///     events.insert(Event::RemovedFromBackpack(Content::None, 0), OxAgSoundConfig::new("assets/default/event/event_remove_from_backpack.ogg"));
///
///     let mut tiles = HashMap::new();
///     tiles.insert(TileType::DeepWater, OxAgSoundConfig::new("assets/default/tile/tile_water.ogg"));
///     tiles.insert(TileType::ShallowWater, OxAgSoundConfig::new("assets/default/tile/tile_water.ogg"));
///     tiles.insert(TileType::Sand, OxAgSoundConfig::new("assets/default/tile/tile_sand.ogg"));
///     tiles.insert(TileType::Grass, OxAgSoundConfig::new("assets/default/tile/tile_grass.ogg"));
///     tiles.insert(TileType::Hill, OxAgSoundConfig::new("assets/default/tile/tile_grass.ogg"));
///     tiles.insert(TileType::Mountain, OxAgSoundConfig::new("assets/default/tile/tile_mountain.ogg"));
///     tiles.insert(TileType::Snow, OxAgSoundConfig::new("assets/default/tile/tile_snow.ogg"));
///     tiles.insert(TileType::Lava, OxAgSoundConfig::new("assets/default/tile/tile_lava.ogg"));
///     tiles.insert(TileType::Teleport(false), OxAgSoundConfig::new("assets/default/tile/tile_teleport.ogg"));
///     tiles.insert(TileType::Street, OxAgSoundConfig::new("assets/default/tile/tile_street.ogg"));
///
///     let mut weather = HashMap::new();
///     weather.insert(WeatherType::Rainy, OxAgSoundConfig::new("assets/default/weather/weather_rainy.ogg"));
///     weather.insert(WeatherType::Foggy, OxAgSoundConfig::new("assets/default/weather/weather_foggy.ogg"));
///     weather.insert(WeatherType::Sunny, OxAgSoundConfig::new("assets/default/weather/weather_sunny.ogg"));
///     weather.insert(WeatherType::TrentinoSnow, OxAgSoundConfig::new("assets/default/weather/weather_winter.ogg"));
///     weather.insert(WeatherType::TropicalMonsoon, OxAgSoundConfig::new("assets/default/weather/weather_tropical.ogg"));
///
///     let audio = OxAgAudioTool::new(events, tiles, weather)?;
///
///     let mut dj = DjRobot {
///         robot: Robot::new(),
///         audio: audio
///     };
///
///     let mut gen = MyGen::new();
///
///     println!("Running!");
///
///     dj.audio.play_audio(&background_music)?;
///
///     let run = Runner::new(Box::new(dj), &mut gen);
///
///     match run {
///         | Ok(mut r) => {
///             let _ = loop {
///                 let _ = r.game_tick();
///                 sleep(Duration::from_millis(10));
///             };
///         }
///         | Err(e) => println!("{:?}", e),
///     }
///
///     Ok(())
/// }
/// ```

use crate::sound_config::OxAgSoundConfig;
use crate::error::error::OxAgAudioToolError;
use crate::util::event_key;
use kira::manager::backend::DefaultBackend;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle};
use kira::tween::Tween;
use robotics_lib::event::events::Event;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::TileType;
use std::collections::HashMap;

/// Struct used to play the various sounds
pub struct OxAgAudioTool {
    /// Maps each [Event] to a [StaticSoundData]
    event_to_sound_data: HashMap<String, StaticSoundData>,
    /// Maps each [TileType] to a [StaticSoundData]
    tile_type_to_sound_data: HashMap<TileType, StaticSoundData>,
    /// Maps each [WeatherType] to a [StaticSoundData]
    weather_type_to_sound_data: HashMap<WeatherType, StaticSoundData>,
    /// The manager for the audio
    audio_manager: AudioManager,
    /// The currently playing sound [StaticSoundHandle] that depends on the given [TileType]
    current_tile_sound: Option<(TileType, StaticSoundHandle)>,
    /// The currently playing sound [StaticSoundHandle] that depends on the given [WeatherType]
    current_weather_sound: Option<(WeatherType, StaticSoundHandle)>,
}

impl OxAgAudioTool {
    /// # Parameters
    /// - event_to_sound_config - a [HashMap] that maps 0 or more [Event]s to a given [OxAgSoundConfig]
    /// - tile_type_to_sound_config - a [HashMap] that maps 0 or more [TileType]s to a given [OxAgSoundConfig]
    /// - weather_type_to_sound_config - a [WeatherType] that maps 0 or more [TileType]s to a given [OxAgSoundConfig]
    pub fn new(
        event_to_sound_config: HashMap<Event, OxAgSoundConfig>,
        tile_type_to_sound_config: HashMap<TileType, OxAgSoundConfig>,
        weather_type_to_sound_config: HashMap<WeatherType, OxAgSoundConfig>,
    ) -> Result<OxAgAudioTool, OxAgAudioToolError> {
        let audio_manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;

        let mut event_to_sound_data: HashMap<String, StaticSoundData> = HashMap::new();
        let mut tile_type_to_sound_data: HashMap<TileType, StaticSoundData> = HashMap::new();
        let mut weather_type_to_sound_data: HashMap<WeatherType, StaticSoundData> =
            HashMap::new();

        for (event, config) in event_to_sound_config.iter() {
            event_to_sound_data.insert(event_key(event).to_string(), config.to_sound_data()?);
        }

        for (tile_type, config) in tile_type_to_sound_config.iter() {
            tile_type_to_sound_data.insert(tile_type.clone(), config.to_sound_data()?);
        }

        for (weather_type, config) in weather_type_to_sound_config.iter() {
            weather_type_to_sound_data.insert(weather_type.clone(), config.to_sound_data()?);
        }

        Ok(OxAgAudioTool {
            event_to_sound_data,
            tile_type_to_sound_data,
            weather_type_to_sound_data,
            audio_manager,
            current_tile_sound: None,
            current_weather_sound: None,
        })
    }

    /// Processes an event and plays the appropriate audio for it
    ///
    /// If it's a [Event::Moved] it will detect whether the [TileType] on which the robot is on changed
    /// and if it did it plays the configured sound for it, stopping the one playing previously for the different [TileType].
    ///
    /// If it's either a [Event::TimeChanged] or a [Event::DayChanged] it will detect whether the [WeatherType] has changed
    /// and if it did it plays the configured sound for it, stopping the one playing previously for the different [WeatherType].
    ///
    /// If it's any other event, it checks whether a sound is configured for it and plays it
    pub fn play_audio_based_on_event(
        &mut self,
        event: &Event,
    ) -> Result<(), OxAgAudioToolError> {
        let event_sound_data = self.event_to_sound_data.get(event_key(event)).cloned();

        if let Some(data) = event_sound_data {
            self.audio_manager.play(data)?;
        }

        match event {
            Event::Moved(tile, _) => {
                let tile_type = tile.tile_type;

                if self.current_tile_sound.is_none() || tile_type != self.current_tile_sound.as_ref().unwrap().0 {
                    if let Some(previous_tile_sound) = &mut self.current_tile_sound {
                        let _ = previous_tile_sound.1.stop(Tween::default());
                    }

                    let sound_data = self.tile_type_to_sound_data.get(&tile_type);

                    if let Some(sound_data) = sound_data {
                        let sound_handle = self
                            .audio_manager
                            .play::<StaticSoundData>(sound_data.clone())?;
                        self.current_tile_sound = Some((tile_type, sound_handle));
                    } else {
                        self.current_tile_sound = None;
                    }
                }
            }
            Event::TimeChanged(environmental_conditions) | Event::DayChanged(environmental_conditions) => {
                let weather_type = environmental_conditions.get_weather_condition();

                if self.current_weather_sound.is_none() || weather_type != self.current_weather_sound.as_ref().unwrap().0 {
                    if let Some(previous_weather_sound) = &mut self.current_weather_sound {
                        let _ = previous_weather_sound.1.stop(Tween::default());
                    }

                    let sound_data = self.weather_type_to_sound_data.get(&weather_type);

                    if let Some(sound_data) = sound_data {
                        let sound_handle = self
                            .audio_manager
                            .play::<StaticSoundData>(sound_data.clone())?;
                        self.current_weather_sound = Some((weather_type, sound_handle));
                    } else {
                        self.current_weather_sound = None;
                    }
                }
            }
            _ => {}
        };

        Ok(())
    }

    /// Plays an audio given some [OxAgSoundConfig]
    pub fn play_audio(
        &mut self,
        sound_config: &OxAgSoundConfig,
    ) -> Result<(), OxAgAudioToolError> {
        let sound_data = sound_config.to_sound_data()?;

        self.audio_manager.play(sound_data)?;

        Ok(())
    }
}