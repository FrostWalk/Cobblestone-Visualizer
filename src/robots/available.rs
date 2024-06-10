use bobot::robot::Bobot;
use log::warn;
use robot_for_visualizer::RobotForVisualizer;
use robotic_ai_prypiat::robot::Scrapbot;
use robotics_lib::runner::Runner;
use robotics_lib::world::world_generator::Generator;
use roomba_robot_test::robot::Roomba;
use strum::{EnumIter, EnumString};

use crate::api::CommonResponse;
use crate::config::CobblestoneConfig;

#[derive(Debug, EnumIter, EnumString)]
pub(crate) enum AvailableRobots {
    Roomba,
    Bobot,
    ScrapBot,
}

impl From<String> for AvailableRobots {
    fn from(s: String) -> Self {
        s.parse().unwrap_or_else(|_| {
            warn!("Invalid robot name: {}", s);
            unreachable!();
        })
    }
}

impl AvailableRobots {
    pub(crate) fn get_runner(s: String, generator: &mut impl Generator) -> Result<Runner, CommonResponse> {
        Scrapbot::set_audio_path(CobblestoneConfig::scrapbot_audio_dir());

        match AvailableRobots::from(s) {
            AvailableRobots::Roomba => {
                match Roomba::get_runner(generator) {
                    Ok(r) => {
                        Ok(r)
                    }
                    Err(e) => {
                        Err(CommonResponse {
                            success: false,
                            msg: Some(format!("{:?}", e)),
                        })
                    }
                }
            }
            AvailableRobots::Bobot => {
                match Bobot::get_runner(generator) {
                    Ok(r) => {
                        Ok(r)
                    }
                    Err(e) => {
                        Err(CommonResponse {
                            success: false,
                            msg: Some(format!("{:?}", e)),
                        })
                    }
                }
            }
            AvailableRobots::ScrapBot => {
                match Scrapbot::get_runner(generator) {
                    Ok(r) => {
                        Ok(r)
                    }
                    Err(e) => {
                        Err(CommonResponse {
                            success: false,
                            msg: Some(format!("{:?}", e)),
                        })
                    }
                }
            }
        }
    }
}