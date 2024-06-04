use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use actix_multipart::Multipart;
use actix_web::{HttpResponse, post};
use actix_web::http::header::ContentType;
use futures_util::stream::StreamExt as _;
use futures_util::TryStreamExt;
use log::info;
use robot_for_visualizer::RobotForVisualizer;
use roomba_robot_test::robot::Roomba;

use crate::api::CommonResponse;
use crate::config::WalleConfig;
use crate::robots::runner::set_robot;
use crate::world_gen_helper::load_world;

#[post("/uploadWorld")]
pub(crate) async fn upload_world(mut payload: Multipart) -> HttpResponse {
    // Create a path to save the file

    // Iterate over multipart stream
    while let Some(item) = match payload.try_next().await {
        Ok(item) => item,
        Err(e) => {
            return HttpResponse::InternalServerError().json(CommonResponse {
                success: false,
                msg: Some(format!("Error processing multipart stream: {}", e)),
            });
        }
    } {
        let mut field = item;
        // Create a file path
        let filepath = PathBuf::from(format!("{}/{}/{}", WalleConfig::static_files_path(), WalleConfig::file_dir(), "wall-e_world.zst"));

        // Create and write to the file
        let mut f = match File::create(filepath) {
            Ok(file) => file,
            Err(e) => {
                return HttpResponse::InternalServerError().json(CommonResponse {
                    success: false,
                    msg: Some(format!("Error creating file: {}", e)),
                });
            }
        };
        while let Some(chunk) = match field.next().await {
            Some(Ok(data)) => Some(data),
            Some(Err(e)) => {
                return HttpResponse::InternalServerError().json(CommonResponse {
                    success: false,
                    msg: Some(format!("Error reading field data: {}", e)),
                });
            }
            None => None,
        } {
            if let Err(e) = f.write_all(&chunk) {
                return HttpResponse::InternalServerError().json(CommonResponse {
                    success: false,
                    msg: Some(format!("Error writing to file: {}", e)),
                });
            }
        }
    }

    let mut wg = match load_world() {
        Ok(w) => { w }
        Err(e) => {
            return HttpResponse::BadRequest().json(CommonResponse {
                success: false,
                msg: Some(format!("{:?}", e)),
            });
        }
    };


    let mut response = CommonResponse {
        success: true,
        msg: None,
    };

    match Roomba::get_runner(&mut wg) {
        Ok(r) => {
            set_robot(r);
        }
        Err(e) => {
            response = CommonResponse {
                success: false,
                msg: Some(format!("{:?}", e)),
            };
            let response = serde_json::to_string(&response).unwrap();
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(response);
        }
    }

    info!("World load completed");

    let response = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}
