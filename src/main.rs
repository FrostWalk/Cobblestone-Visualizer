mod robot;
mod world_generator;

use std::{thread, time};
use bevy::prelude::App;
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use robotics_lib::runner::{Robot, Runner};
use crate::robot::Roomba;

fn main() {
    
    let mut generator = OxAgWorldGenerator::builder().load("world").unwrap();

    let mut run = Runner::new(
        Box::new(Roomba {
            robot: Robot::new(),
        }),
        &mut generator,
    )
        .unwrap();

    App::new().run();

    loop {
        match run.game_tick() {
            Ok(_) => {}
            Err(e) => { println!("{:?}", e); }
        }
        thread::sleep(time::Duration::from_millis(500));

    }
}

