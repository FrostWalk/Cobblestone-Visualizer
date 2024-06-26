use std::ops::Deref;
use std::sync::atomic::Ordering::Relaxed;

use actix::ActorContext;
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use actix_web_actors::ws::Message::{Nop, Text};
use bytestring::ByteString;
use common_messages::events::LibEvent;
use common_messages::messages::{Environment, Response};
use log::warn;
use robot_for_visualizer::{get_day_periods, get_event_from_queue, get_time, get_weather_condition, get_world_map};

use crate::robots::runner_logic::get_robot_data;
use crate::robots::variables::PAUSE;
use crate::websocket::errors::CobblestoneError;
use crate::websocket::update_socket::UpdateSocket;

pub(crate) fn create_update() -> Result<Message, ProtocolError> {
    let data = get_robot_data();
    if data.is_none() || PAUSE.load(Relaxed) {
        return Ok(Nop);
    }

    let env = Environment::new(get_time(), get_weather_condition(), get_day_periods());
    let map = get_world_map().deref().clone();
    let event = match get_event_from_queue() {
        None => { String::new() }
        Some(e) => {
            LibEvent::from(e).to_string()
        }
    };
    let response = Response::new(event, data.unwrap(), env, map).to_json().unwrap();

    Ok(Text(ByteString::from(response)))
}

pub(crate) fn updates_handler(msg: Result<Message, ProtocolError>, ctx: &mut WebsocketContext<UpdateSocket>) {
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
            ctx.text(CobblestoneError::bin_data_not_supported());
            warn!("Unexpected binary data received");
        }
        Message::Continuation(_) => {
            ctx.text(CobblestoneError::frag_not_supported());
            warn!("Fragmentation received")
        }
        Message::Ping(m) => { ctx.pong(&m) }
        Message::Pong(m) => { ctx.ping(&m) }
        Message::Close(reason) => {
            ctx.close(reason);
            ctx.stop();
        }
        Nop => {}
    }
}
