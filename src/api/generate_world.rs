use actix_web::{HttpResponse, post, web};
use actix_web::http::header::ContentType;
use log::info;
use serde::Deserialize;

use crate::api::CommonResponse;
use crate::api::get_available_robots::AvailableRobots;
use crate::robots::runner_logic::{set_robot, set_wait};
use crate::world_gen_helper::get_generator;

#[derive(Deserialize)]
struct WorldData {
    #[serde(alias = "worldSize")]
    world_size: usize,
    seed: u64,
    wait: u64,
    robot: String,
}

#[post("/generate")]
async fn generate_world(data: web::Json<WorldData>) -> HttpResponse {
    let req = data.into_inner();

    info!(
        "Generating World: {}, seed: {}, wait: {}, robot: {}",
        req.world_size, req.seed, req.wait, req.robot
    );

    let mut response = CommonResponse {
        success: true,
        msg: None,
    };

    set_wait(req.wait);
    let mut generator = match get_generator(req.world_size, req.seed) {
        Ok(g) => { g }
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
    };

    let runner = match AvailableRobots::get_runner(req.robot, &mut generator) {
        Ok(r) => { r }
        Err(e) => {
            let response = serde_json::to_string(&e).unwrap();
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(response);
        }
    };

    set_robot(runner);
    
    info!("World generation completed");

    let response = serde_json::to_string(&response).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}