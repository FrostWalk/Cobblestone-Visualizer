use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use derive_new::new;
use serde::Serialize;

use crate::world_gen_helper::get_random_seed;

#[derive(Serialize, new)]
struct SeedResponse {
    seed: u64,
}

#[get("/randomSeed")]
pub(crate) async fn generate_seed() -> impl Responder {
    let response = serde_json::to_string(&SeedResponse::new(get_random_seed())).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}