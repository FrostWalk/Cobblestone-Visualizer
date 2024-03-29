use robotics_lib::event::events::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum ApplicationState {
    Start,
    Pause,
    Stop,
}

#[derive(Serialize, Deserialize)]
struct Controls {
    state: ApplicationState,
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    event: Event,
}