use actix_web::{App, HttpServer};

use crate::config::WalleConfig;
use crate::static_files::static_files;

mod static_files;
mod config;
mod websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    WalleConfig::load();
    println!("Starting server at http://{}:{}/", WalleConfig::address(), WalleConfig::port());


    let a = (WalleConfig::address(), WalleConfig::port());
    HttpServer::new(|| {
        App::new()
            .service(static_files())
    }).bind(a)?.run().await
}