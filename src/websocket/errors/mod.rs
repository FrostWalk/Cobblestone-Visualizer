use serde::{Deserialize, Serialize};
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

impl WalleError {
    pub fn bin_data_not_supported() -> String {
        WalleError::from_str("Binary data not supported")
    }
}

impl WalleError {
    pub fn command_not_supported() -> String {
        WalleError::from_str("The command is not supported")
    }
}

impl WalleError {
    pub fn frag_not_supported() -> String {
        WalleError::from_str("This websocket does not support fragmentation")
    }
}
