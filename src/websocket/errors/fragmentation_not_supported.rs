use crate::websocket::errors::WalleError;

impl WalleError {
    pub fn frag_not_supported() -> String {
        WalleError::create("This websocket does not support fragmentation")
    }
}
