use crate::websocket::errors::WalleError;

impl WalleError {
    pub fn bin_data() -> String {
        WalleError::create("Binary data not supported")
    }
}