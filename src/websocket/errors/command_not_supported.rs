use crate::websocket::errors::WalleError;

impl WalleError {
    pub fn command_not_supported() -> String {
        WalleError::from_str("The command is not supported")
    }
}