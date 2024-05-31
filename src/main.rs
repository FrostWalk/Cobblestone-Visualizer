use actix_web::{App, HttpServer, web};
use log::info;

use crate::config::WalleConfig;
use crate::static_files::static_files;
use crate::websocket::walle_web_socket::walle_web_socket;

mod static_files;
mod config;
mod websocket;
mod robots;
mod world_gen_helper;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    WalleConfig::load();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting server at http://{}:{}/", WalleConfig::address(), WalleConfig::port());

    HttpServer::new(move || {
        App::new()
            .route("/events", web::get().to(walle_web_socket))
            .service(static_files())
    }).bind((WalleConfig::address(), WalleConfig::port()))?.run().await
}