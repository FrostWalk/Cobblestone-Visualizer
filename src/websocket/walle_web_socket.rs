use actix::{Actor, ActorContext, StreamHandler};
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use actix_web_actors::ws::Message;
use log::warn;

use crate::websocket::errors::WalleError;
use crate::websocket::handler::close::closing_handler;
use crate::websocket::handler::text::text_handler;

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
            Message::Text(message) => { text_handler(message, ctx) }
            Message::Binary(_) => {
                ctx.text(WalleError::bin_data_not_supported());
                warn!("Unexpected binary data received");
            }
            Message::Continuation(_) => {
                ctx.text(WalleError::frag_not_supported());
                warn!("Fragmentation received")
            }
            Message::Ping(_) => { ctx.text("pong") }
            Message::Pong(_) => { ctx.text("ping") }
            Message::Close(reason) => { closing_handler(reason, ctx); }
            Message::Nop => {}
        }
    }
}

pub(crate) async fn walle_web_socket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WalleWebSocket {}, &req, stream);
    println!("{:?}", resp);
    resp
}