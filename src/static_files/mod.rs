use actix_files::Files;

use crate::config::CobblestoneConfig;

pub fn static_files() -> Files {
    Files::new("/", CobblestoneConfig::static_files_path()).index_file(CobblestoneConfig::index())
}