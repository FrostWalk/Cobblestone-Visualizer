use actix::{Actor, ActorContext, StreamHandler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message;

use crate::websocket::errors::WalleError;

pub struct WalleWebSocket;

impl Actor for WalleWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<Message, ws::ProtocolError>> for WalleWebSocket {
    fn handle(&mut self, msg: Result<Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            Message::Text(txt) => {}
            Message::Binary(_) => {
                ctx.text(WalleError::bin_data());
                println!("Unexpected binary data received")
            }
            Message::Continuation(_) => { ctx.text(WalleError::frag_not_supported()) }
            Message::Ping(_) => {}
            Message::Pong(_) => {}
            Message::Close(c) => {}
            Message::Nop => {}
        }
    }
}