use std::fmt::{Display, Formatter};

use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use serde::Serialize;

pub(crate) mod generate_world;
pub(crate) mod random_seed;
pub(crate) mod get_available_robots;

#[derive(Debug, Serialize)]
pub(crate) struct ResponseErr {
    err: String,
}

impl Display for ResponseErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ResponseErr {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(&self).unwrap();
        let res = HttpResponse::new(self.status_code());
        res.set_body(BoxBody::new(body))
    }
}