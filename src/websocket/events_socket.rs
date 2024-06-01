use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws::{self, Message, ProtocolError};
use bytestring::ByteString;
use futures_util::stream::StreamExt;
use log::{error, warn};
use roomba_robot_test::robot::Roomba;
use tokio_stream::wrappers::IntervalStream;

use crate::robots::robot::get_wait;
use crate::websocket::errors::WalleError;

struct EventsSocket {}

impl EventsSocket {
    fn start_stream(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let interval = tokio::time::interval(get_wait() / 4);
        let interval_stream = IntervalStream::new(interval).map(|_| {
            if let Some(event) = Roomba::get_event_from_queue() {
                Ok(Message::Text(ByteString::from(event.to_string())))
            } else {
                Ok(Message::Text(ByteString::from("")))
            }
        });

        ctx.add_stream(interval_stream);
    }
}

impl Actor for EventsSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_stream(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ProtocolError>> for EventsSocket {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, socket: &mut Self::Context) {
        let msg = match msg {
            Err(e) => {
                socket.stop();
                error!("Command socekt error: {}",e);
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            Message::Text(m) => {
                socket.text(m);
            }
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

pub(crate) async fn events_socket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(EventsSocket {}, &req, stream)
}