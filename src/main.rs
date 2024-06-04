use actix_web::{App, HttpServer, web};
use log::info;

use crate::api::generate_and_download::generate_and_download;
use crate::api::generate_world::generate_world;
use crate::api::get_available_robots::get_available_robots;
use crate::api::random_seed::generate_seed;
use crate::config::WalleConfig;
use crate::static_files::static_files;
use crate::websocket::commands_socket::commands_socket;
use crate::websocket::update_socket::update_socket;

mod static_files;
mod config;
mod websocket;
mod robots;
mod world_gen_helper;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    WalleConfig::load();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting server at http://{}:{}/", WalleConfig::address(), WalleConfig::port());

    HttpServer::new(move || {
        App::new()
            .route("/commands", web::get().to(commands_socket))
            .route("/updates", web::get().to(update_socket))
            .service(generate_seed)
            .service(get_available_robots)
            .service(generate_world)
            .service(generate_and_download)
            .service(static_files())
    }).bind((WalleConfig::address(), WalleConfig::port()))?.run().await
}