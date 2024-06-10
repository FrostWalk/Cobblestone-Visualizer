use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::Serialize;
use strum::IntoEnumIterator;

use crate::robots::available::AvailableRobots;

#[derive(Serialize)]
struct RobotsResponse {
    robots: Vec<String>,
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
