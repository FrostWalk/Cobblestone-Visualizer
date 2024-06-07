use actix_web::{get, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::Serialize;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Serialize)]
struct RobotsResponse {
    robots: Vec<String>,
}

#[derive(Debug, EnumIter)]
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
    fn from(value: String) -> Self {
        match value.as_str() {
            "Roomba" => { AvailableRobots::Roomba }
            "Bobot" => { AvailableRobots::Bobot }
            "Matteo" => { AvailableRobots::ScrapBot }
            _ => {
                unreachable!()
            }
        }
    }
}