use actix_files::Files;
use actix_web::{App, HttpServer};

use crate::static_files::static_files;

mod static_files;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(static_files())
    }).bind(("127.0.0.1", 8080))?.run().await
}