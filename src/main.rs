use actix_web::{App, HttpServer};
use log::info;

use crate::config::WalleConfig;
use crate::static_files::static_files;

mod static_files;
mod config;
mod websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    WalleConfig::load();
    info!("Starting server at http://{}:{}/", WalleConfig::address(), WalleConfig::port());

    HttpServer::new(|| {
        App::new()
            .service(static_files())
    }).bind((WalleConfig::address(), WalleConfig::port()))?.run().await
}