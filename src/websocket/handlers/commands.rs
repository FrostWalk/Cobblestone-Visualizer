use actix_web_actors::ws::WebsocketContext;
use bytestring::ByteString;
use common_messages::messages::{Command, Request};
use log::error;

use crate::robots::runner::{pause_robot, resume_robot, run_robot, stop_robot};
use crate::websocket::commands_socket::CommandsSocket;
use crate::websocket::errors::CobblestoneError;

pub(crate) fn commands_handler(payload: ByteString, socket: &mut WebsocketContext<CommandsSocket>) {
    let request = match Request::from_json(payload.as_ref()) {
        Ok(m) => { m }
        Err(e) => {
            error!("{}",e);
            socket.text(CobblestoneError::from_string(e.to_string()));
            return;
        }
    };

    match request.command() {
        Command::Start => { run_robot(); }
        Command::Stop => { stop_robot() }
        Command::Pause => { pause_robot() }
        Command::Resume => { resume_robot() }
    }
}