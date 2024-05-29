use crate::websocket::errors::WalleError;

impl WalleError {
    pub fn bin_data() -> String {
        WalleError::new("Binary data not supported").as_json()
    }
}