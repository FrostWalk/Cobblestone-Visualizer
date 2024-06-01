use actix::{Actor, ActorContext, StreamHandler};
use actix::fut::err;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use actix_web_actors::ws::Message;
use log::{error, warn};

use crate::websocket::errors::WalleError;
use crate::websocket::handlers::commands::commands_handler;

pub(crate) struct CommandsSocket;

impl Actor for CommandsSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<Message, ws::ProtocolError>> for CommandsSocket {
    fn handle(&mut self, msg: Result<Message, ws::ProtocolError>, socket: &mut Self::Context) {
        let msg = match msg {
            Err(e) => {
                socket.stop();
                error!("Command socekt error: {}",e);
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            Message::Text(message) => { commands_handler(message, socket) }
            Message::Binary(_) => {
                socket.text(WalleError::bin_data_not_supported());
                warn!("Unexpected binary data received");
            }
            Message::Continuation(_) => {
                socket.text(WalleError::frag_not_supported());
                warn!("Fragmentation received")
            }
            Message::Ping(m) => { socket.pong(&m) }
            Message::Pong(m) => { socket.ping(&m) }
            Message::Close(reason) => {
                socket.close(reason);
                socket.stop();
            }
            Message::Nop => {}
        }
    }
}

pub(crate) async fn commands_socket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(CommandsSocket {}, &req, stream)
}