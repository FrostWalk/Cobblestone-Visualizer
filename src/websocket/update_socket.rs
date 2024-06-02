use std::process::exit;

use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws::{self, Message, ProtocolError};
use actix_web_actors::ws::Message::{Nop, Text};
use bytestring::ByteString;
use common_messages::events::LibEvent;
use futures_util::stream::StreamExt;
use log::{error, warn};
use robotics_lib::event::events::Event;
use roomba_robot_test::robot::Roomba;
use tokio_stream::wrappers::IntervalStream;

use crate::robots::robot::{get_robot_data, get_wait};
use crate::websocket::errors::WalleError;

struct UpdateSocket {}

impl UpdateSocket {
    fn start_stream(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let interval = tokio::time::interval(get_wait());
        let interval_stream = IntervalStream::new(interval).map(|_| {
            if let Some(event) = Roomba::get_event_from_queue() {
                create_update(event)
            } else {
                Ok(Nop)
            }
        });

        ctx.add_stream(interval_stream);
    }
}

impl Actor for UpdateSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_stream(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ProtocolError>> for UpdateSocket {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            Text(m) => {
                ctx.text(m);
            }
            Message::Binary(_) => {
                ctx.text(WalleError::bin_data_not_supported());
                warn!("Unexpected binary data received");
            }
            Message::Continuation(_) => {
                ctx.text(WalleError::frag_not_supported());
                warn!("Fragmentation received")
            }
            Message::Ping(m) => { ctx.pong(&m) }
            Message::Pong(m) => { ctx.ping(&m) }
            Message::Close(reason) => { ctx.close(reason); ctx.stop(); }
            Message::Nop => {}
        }
    }
}

pub(crate) async fn update_socket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(UpdateSocket {}, &req, stream)
}

fn create_update(event: Event) -> Result<Message, ProtocolError> {
    let event = LibEvent::from(event);
    let data = get_robot_data();
    let response = match common_messages::messages::Response::new(event, data).to_json() {
        Ok(m) => { m }
        Err(e) => {
            error!("{}",e);
            exit(0)
        }
    };
    Ok(Text(ByteString::from(response)))
}