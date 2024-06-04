use actix_web::{HttpResponse, post, web};
use actix_web::http::header::ContentType;
use log::info;
use serde::Deserialize;
use crate::api::CommonResponse;

use crate::world_gen_helper::generate_and_save;

#[derive(Deserialize)]
struct WorldData {
    #[serde(alias = "worldSize")]
    world_size: usize,
    seed: u64,
}

#[post("/downloadWorld")]
async fn generate_and_download(data: web::Json<WorldData>) -> HttpResponse {
    let req = data.into_inner();

    info!("Generating World for download: {}, seed: {}",req.world_size, req.seed);

    match generate_and_save(req.world_size, req.seed) {
        Ok(_) => {}
        Err(e) => {
            let response = CommonResponse {
                success: false,
                msg: Some(e),
            };
            return HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&response).unwrap());
        }
    };

    info!("World generation completed");

    let response = CommonResponse {
        success: true,
        msg: None,
    };
    HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&response).unwrap())
}