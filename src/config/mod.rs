use std::sync::{Arc, RwLock};

use figment::{Figment, providers::{Env, Format, Toml}};
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CobblestoneConfig {
    address: String,
    port: u16,
    static_files_path: String,
    index: String,
    file_dir: String,
    scrapbot_audio_dir: String,
}

lazy_static! {
    static ref CONFIG: Arc<RwLock<CobblestoneConfig>> = Arc::new(RwLock::new(CobblestoneConfig::load()));
}

impl CobblestoneConfig {
    pub(crate) fn load() -> Self {
        Figment::new()
            .merge(Env::prefixed("COBBLESTONE_"))
            .merge(Toml::file("config.toml"))
            .extract().expect("Failed to load configuration")
    }
    pub(crate) fn address() -> String {
        CONFIG.read().expect("Unable to lock CONFIG").address.clone()
    }

    pub(crate) fn port() -> u16 {
        CONFIG.read().expect("Unable to lock CONFIG").port
    }

    pub(crate) fn static_files_path() -> String {
        CONFIG.read().expect("Unable to lock CONFIG").static_files_path.clone()
    }

    pub(crate) fn index() -> String {
        CONFIG.read().expect("Unable to lock CONFIG").index.clone()
    }
    pub(crate) fn file_dir() -> String { CONFIG.read().expect("Unable to lock CONFIG").file_dir.clone() }
    pub(crate) fn scrapbot_audio_dir() -> String { CONFIG.read().expect("Unable to lock CONFIG").scrapbot_audio_dir.clone() }
}