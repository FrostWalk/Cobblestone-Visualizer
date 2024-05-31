use actix_web_actors::ws::CloseReason;

use crate::websocket::walle_web_socket::WalleWebSocket;

pub(crate) fn closing_handler(reason: Option<CloseReason>, ctx: &mut <WalleWebSocket as actix::Actor>::Context){
    
}