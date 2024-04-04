use std::sync::{Arc, Mutex};

use message_io::network::{NetEvent, Transport};
use message_io::node::{self};
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::Foggy;
use robotics_lib::world::tile::{Content, Tile, TileType};

use walle_visualizer::messages::{Command, Response, Status};

use crate::world::generate;

mod world;

const ADDRESS: &str = "0.0.0.0:3439";

async fn run() {
    let (handler, listener) = node::split::<()>();

    let world_mutex = Arc::new(Mutex::new((
        vec![vec![Tile { tile_type: TileType::Grass, content: Content::None, elevation: 0 }; 1]; 1],
        (0, 0),
        EnvironmentalConditions::new(&[Foggy], 0, 0).unwrap(), 0.0,
        None)));

    if let Err(e) = handler.network().listen(Transport::Tcp, ADDRESS) {
        return println!("Can not listening at {}\n{}", ADDRESS, e);
    }

    listener.for_each(move |event| {
        match event.network() {
            NetEvent::Connected(a, _) => {
                println!("client connected: {}", a);
            }
            NetEvent::Message(endpoint, input_data) => {
                let message: Command = bincode::deserialize(input_data).unwrap();
                match message {
                    Command::CreateWorld(s) => {
                        println!("generating world");
                        
                        let mut response = bincode::serialize(&Response::Status(Status::StartedWorldGen)).unwrap();
                        handler.network().send(endpoint, &response);
                        generate(s, world_mutex.clone());

                        response = bincode::serialize(&Response::Status(Status::FinishedWorldGen)).unwrap();
                        handler.network().send(endpoint, &response);
                        println!("finished: generating world");

                    }
                    Command::Start => {}
                    Command::Pause => {}
                    Command::Stop => {}
                    Command::ExportWorld => {
                        let output_data = bincode::serialize(&message).unwrap();
                        handler.network().send(endpoint, &output_data);
                    }
                }
            }
            NetEvent::Disconnected(_) => (),
            _ => ()
        }
    }
    );
}

#[tokio::main]
async fn main() {
    run().await;
}