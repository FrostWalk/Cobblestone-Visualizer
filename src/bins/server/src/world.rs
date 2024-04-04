use std::ops::{DerefMut};
use std::sync::{Arc, Mutex};

use exclusion_zone::generator::{GenResult, WorldGenerator};
use robotics_lib::world::world_generator::Generator;

pub fn generate(s: Option<Box<WorldGenerator>>, w: Arc<Mutex<GenResult>>) {
    *w.lock().unwrap().deref_mut() = s.unwrap().gen()
}