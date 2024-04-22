mod robot;
mod world_generator;

use std::{thread, time};
use bevy::prelude::App;
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use robotics_lib::runner::{Robot, Runner};
use crate::robot::Roomba;

fn main() {
    
    let mut generator = OxAgWorldGenerator::builder().load("world").unwrap();
    
    App::new().run();

   
}

