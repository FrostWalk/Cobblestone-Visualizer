use serde::Serialize;

pub(crate) mod generate_world;
pub(crate) mod random_seed;
pub(crate) mod get_available_robots;
pub(crate) mod generate_and_download;
pub(crate) mod upload_world;

#[derive(Serialize)]
pub(crate) struct CommonResponse {
    pub(crate) success: bool,
    pub(crate) msg: Option<String>,
}