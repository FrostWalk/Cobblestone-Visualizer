use serde::{Deserialize, Serialize};

pub mod bin_data_error;
pub mod fragmentation_not_supported;

#[derive(Serialize, Deserialize)]
pub struct WalleError {
    msg: String,
}

impl WalleError {
    pub(crate) fn new(m: &str) -> Self {
        Self {
            msg: m.to_string()
        }
    }
    fn as_json(&self) -> String {
        serde_json::to_string(self).expect("Unable to parse Error struct into json")
    }
}