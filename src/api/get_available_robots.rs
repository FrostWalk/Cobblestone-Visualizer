use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use log::warn;
use robot_for_visualizer::RobotForVisualizer;
use robotic_ai_prypiat::bot::Scrapbot;
use robotics_lib::runner::Runner;
use robotics_lib::world::world_generator::Generator;
use roomba_robot_test::robot::Roomba;
use serde::Serialize;
use strum::{EnumIter, EnumString, IntoEnumIterator};

use crate::api::CommonResponse;

#[derive(Serialize)]
struct RobotsResponse {
    robots: Vec<String>,
}

#[derive(Debug, EnumIter, EnumString)]
pub(crate) enum AvailableRobots {
    Roomba,
    Bobot,
    ScrapBot,
}

#[get("/robots")]
pub(crate) async fn get_available_robots() -> impl Responder {
    let robots = RobotsResponse {
        robots: AvailableRobots::iter().map(|e| format!("{:?}", e)).collect(),
    };

    let response = serde_json::to_string(&robots).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
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
                Err(CommonResponse {
                    success: false,
                    msg: Some(String::from("Robot not available")),
                })
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