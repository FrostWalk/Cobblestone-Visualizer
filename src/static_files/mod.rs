use actix_files::Files;

use crate::config::WalleConfig;

pub fn static_files() -> Files {
    Files::new("/", WalleConfig::static_files_path()).index_file(WalleConfig::index())
}