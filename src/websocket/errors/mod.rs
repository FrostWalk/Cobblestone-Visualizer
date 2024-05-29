use serde::{Deserialize, Serialize};

pub mod bin_data_error;
pub mod fragmentation_not_supported;

#[derive(Serialize, Deserialize)]
pub struct WalleError {
    msg: String,
}

impl WalleError {
    pub(crate) fn create(m: &str) -> String {
        serde_json::to_string(&Self {
            msg: m.to_string()
        }).expect("unable to jesonify error")
    }
}