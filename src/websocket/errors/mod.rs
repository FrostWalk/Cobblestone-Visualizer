use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct CobblestoneError {
    msg: String,
}

impl CobblestoneError {
    pub(crate) fn from_str(m: &str) -> String {
        Self::from_string(m.to_string())
    }

    pub(crate) fn from_string(m: String) -> String {
        serde_json::to_string(&Self {
            msg: m
        }).expect("unable to jesonify error")
    }
}

impl CobblestoneError {
    pub fn bin_data_not_supported() -> String {
        CobblestoneError::from_str("Binary data not supported")
    }
}

impl CobblestoneError {
    pub fn command_not_supported() -> String {
        CobblestoneError::from_str("The command is not supported")
    }
}

impl CobblestoneError {
    pub fn frag_not_supported() -> String {
        CobblestoneError::from_str("This websocket does not support fragmentation")
    }
}
