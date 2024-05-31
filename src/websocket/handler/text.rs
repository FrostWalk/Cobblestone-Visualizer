use bytestring::ByteString;
use common_messages::messages::{Command, Request};
use log::error;

use crate::websocket::errors::WalleError;
use crate::websocket::walle_web_socket::WalleWebSocket;

pub(crate) fn text_handler(payload: ByteString, ctx: &mut <WalleWebSocket as actix::Actor>::Context) {
    let request = match Request::from_json(payload.as_ref()) {
        Ok(m) => { m }
        Err(e) => {
            error!("{}",e);
            ctx.text(WalleError::from_string(e.to_string()));
            return;
        }
    };

    match request.command() {
        Command::Start => {run_robot()}
        Command::Stop => {}
        Command::Reset => {}
        Command::Pause => {}
        Command::Resume => {}
    }
}