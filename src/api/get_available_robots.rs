use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::Serialize;

#[derive(Serialize)]
struct RobotsResponse<'a> {
    robots: Vec<&'a str>,
}

#[get("/robots")]
pub(crate) async fn get_available_robots() -> impl Responder {
    let robots = RobotsResponse {
        robots: vec!["Roomba"]
    };

    let response = serde_json::to_string(&robots).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}