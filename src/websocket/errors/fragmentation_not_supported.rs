use crate::websocket::errors::WalleError;

impl WalleError {
    pub fn frag_not_supported() -> String {
        WalleError::new("This websocket does not support fragmentation").as_json()
    }
}
