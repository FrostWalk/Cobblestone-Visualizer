use crate::websocket::errors::WalleError;

impl WalleError {
    pub fn bin_data_not_supported() -> String {
        WalleError::create("Binary data not supported")
    }
}