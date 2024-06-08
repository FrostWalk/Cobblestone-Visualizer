use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws::{self, Message, ProtocolError};
use futures_util::stream::StreamExt;
use tokio_stream::wrappers::IntervalStream;

use crate::robots::runner_logic::get_wait;
use crate::websocket::handlers::updates::{create_update, updates_handler};

pub(crate) struct UpdateSocket {}

impl UpdateSocket {
    fn start_stream(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let interval = tokio::time::interval(get_wait());
        let interval_stream = IntervalStream::new(interval).map(|_| {
            create_update()
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
        updates_handler(msg, ctx)
    }
}

pub(crate) async fn update_socket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(UpdateSocket {}, &req, stream)
}
