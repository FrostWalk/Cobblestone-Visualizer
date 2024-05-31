use crate::websocket::errors::WalleError;

impl WalleError {
    pub fn bin_data_not_supported() -> String {
        WalleError::from_str("Binary data not supported")
    }
}