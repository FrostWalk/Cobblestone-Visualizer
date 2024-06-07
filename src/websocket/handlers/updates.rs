use std::ops::Deref;

use actix::ActorContext;
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use actix_web_actors::ws::Message::{Nop, Text};
use bytestring::ByteString;
use common_messages::events::LibEvent;
use common_messages::messages::{Environment, Response};
use log::{info, warn};
use robot_for_visualizer::{get_day_periods, get_event_from_queue, get_time, get_weather_condition, get_world_map};

use crate::robots::runner::get_robot_data;
use crate::websocket::errors::CobblestoneError;
use crate::websocket::update_socket::UpdateSocket;

pub(crate) fn create_update() -> Result<Message, ProtocolError> {
    let data = get_robot_data();
    let env = Environment::new(get_time(), get_weather_condition(), get_day_periods());
    let map = get_world_map().deref().clone();

    let event = get_event_from_queue();

/*    if map.iter().any(|row| row.iter().any(|cell| cell.is_some())) {
        info!("Some");
    }
*/    
    let response = if event.is_some() {
        let event = LibEvent::from(event.unwrap());

        let event = match event {
            LibEvent::Ready => {
                Some(LibEvent::Ready)
            }
            LibEvent::Terminated => {
                Some(LibEvent::Terminated)
            }
            _ => {
                None
            }
        };
        Response::new(event, data, env, map).to_json().unwrap()
    } else {
        Response::new(None, data, env, map).to_json().unwrap()
    };

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
