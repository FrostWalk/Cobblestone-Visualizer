use actix_web_actors::ws::WebsocketContext;
use bytestring::ByteString;
use common_messages::messages::{Command, Request};
use log::error;
use robot_for_visualizer::RobotForVisualizer;
use roomba_robot_test::robot::Roomba;

use crate::robots::robot::{pause_robot, resume_robot, run_robot, set_robot, stop_robot};
use crate::websocket::commands_socket::CommandsSocket;
use crate::websocket::errors::WalleError;
use crate::world_gen_helper::get_generator;

pub(crate) fn commands_handler(payload: ByteString, socket: &mut WebsocketContext<CommandsSocket>) {
    let request = match Request::from_json(payload.as_ref()) {
        Ok(m) => { m }
        Err(e) => {
            error!("{}",e);
            socket.text(WalleError::from_string(e.to_string()));
            return;
        }
    };

    match request.command() {
        Command::Start => {
            set_robot(Roomba::get_runner(&mut get_generator(100, 0)));
            run_robot();
        }
        Command::Stop => { stop_robot() }
        Command::Reset => {}
        Command::Pause => { pause_robot() }
        Command::Resume => { resume_robot() }
    }
}