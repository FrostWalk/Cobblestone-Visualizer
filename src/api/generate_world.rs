use actix_web::{HttpResponse, post, web};
use actix_web::http::header::ContentType;
use log::info;
use oxagworldgenerator::world_generator::content_options::OxAgContentOptions;
use robotics_lib::world::tile::Content;
use serde::Deserialize;

use crate::api::CommonResponse;
use crate::robots::available::AvailableRobots;
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

    let content = match AvailableRobots::from(req.robot.clone()) {
        AvailableRobots::Roomba => {
            None
        }
        AvailableRobots::Bobot => { None }
        AvailableRobots::ScrapBot => {
            Some(vec![
                (
                    Content::Garbage(1),
                    OxAgContentOptions {
                        in_batches: true,
                        is_present: true,
                        min_spawn_number: 4,
                        max_radius: 2,
                        with_max_spawn_number: true,
                        max_spawn_number: 20,
                        percentage: 1f64,
                    },
                ),
                (
                    Content::Bin(0..1),
                    OxAgContentOptions {
                        in_batches: false,
                        is_present: true,
                        min_spawn_number: 3,
                        max_radius: 0,
                        with_max_spawn_number: true,
                        max_spawn_number: 3,
                        percentage: 1f64,
                    },
                ),
            ])
        }
    };

    let mut generator = match get_generator(req.world_size, req.seed, content) {
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