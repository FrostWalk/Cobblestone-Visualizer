use std::fs::File;
use std::io::Write;
use std::path::Path;

use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse, post};
use actix_web::http::header::ContentType;
use futures_util::{StreamExt, TryStreamExt};
use robot_for_visualizer::RobotForVisualizer;
use roomba_robot_test::robot::Roomba;

use crate::api::CommonResponse;
use crate::api::get_available_robots::AvailableRobots;
use crate::config::CobblestoneConfig;
use crate::robots::runner::set_robot;
use crate::world_gen_helper::load_world;

#[post("/uploadWorld")]
pub(crate) async fn upload_world(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut robot_name = String::new();
    let mut file_name: String = String::from("");

    // Iterate over multipart stream
    while let Some(item) = payload.try_next().await? {
        let mut field = item;

        // Handle regular form fields
        let field_name = field.name();
        if field_name == "robot" {
            // Extract the robot name from the form field
            let mut bytes = Vec::new();
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                bytes.extend_from_slice(&data);
            }
            robot_name.clone_from(&String::from_utf8(bytes).unwrap_or_default());
        }

        let content_disposition = field.content_disposition();
        // Estraiamo il filename se presente
        if let Some(filename) = content_disposition.get_filename() {
            let path = Path::new(CobblestoneConfig::static_files_path().as_str())
                .join(CobblestoneConfig::file_dir().as_str()).join(filename);
            file_name = filename.to_string();
            let mut file = File::create(path)?;
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                file.write_all(&data)?;
            }
        }
    }


    let mut response = CommonResponse {
        success: true,
        msg: None,
    };

    let mut generator = match load_world(file_name) {
        Ok(g) => { g }
        Err(e) => {
            response = CommonResponse {
                success: false,
                msg: Some(format!("{:?}", e)),
            };
            let response = serde_json::to_string(&response).unwrap();
            return Ok(HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(response));
        }
    };

    match AvailableRobots::from(robot_name) {
        AvailableRobots::Roomba => {
            match Roomba::get_runner(&mut generator) {
                Ok(r) => {
                    set_robot(r);
                }
                Err(e) => {
                    response = CommonResponse {
                        success: false,
                        msg: Some(format!("{:?}", e)),
                    }
                }
            }
        }
        AvailableRobots::Bobot => {}
        AvailableRobots::Matteo => {}
    }

    Ok(HttpResponse::Ok().json(response))
}


