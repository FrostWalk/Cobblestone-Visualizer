use crate::websocket::errors::WalleError;

impl WalleError {
    pub fn frag_not_supported() -> String {
        WalleError::from_str("This websocket does not support fragmentation")
    }
}
