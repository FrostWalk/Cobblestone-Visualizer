use exclusion_zone::generator::WorldGenerator;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Command {
    CreateWorld(Option<Box<WorldGenerator>>),
    Start,
    Pause,
    Stop,
    ExportWorld,
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Event(robotics_lib::event::events::Event),
    World(Result<Vec<u8>, String>),
    Info(RobotDetails),
    Status(Status),
    Error(String)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    StartedWorldGen,
    FinishedWorldGen,
}

#[derive(Serialize, Deserialize)]
pub struct RobotDetails {
    robot_name: String,
    tools: Vec<String>,
}
