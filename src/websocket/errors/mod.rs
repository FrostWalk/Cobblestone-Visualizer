use serde::{Deserialize, Serialize};

pub mod bin_data_not_supported;
pub mod fragmentation_not_supported;
mod command_not_supported;

#[derive(Serialize, Deserialize)]
pub struct WalleError {
    msg: String,
}

impl WalleError {
    pub(crate) fn from_str(m: &str) -> String {
        Self::from_string(m.to_string())
    }

    pub(crate) fn from_string(m: String) -> String {
        serde_json::to_string(&Self {
            msg: m
        }).expect("unable to jesonify error")
    }
}