use std::fs::File;
use std::io::{Read, Write};

use oxagworldgenerator::world_generator::presets::content_presets::OxAgContentPresets;
use oxagworldgenerator::world_generator::presets::tile_type_presets::OxAgTileTypePresets;
use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use rand::{RngCore, thread_rng};
use robotics_lib::world::world_generator::Generator;

pub(crate) fn get_generator(size: usize, seed: u64) -> impl Generator {
    OxAgWorldGeneratorBuilder::new()
        .set_seed(seed)
        .set_size(size)
        .set_tile_type_options_from_preset(OxAgTileTypePresets::Default)
        .set_content_options_from_preset(OxAgContentPresets::Default)
        .build()
        .expect("Unable to create a world generator")
}

#[inline(always)]
pub(crate) fn get_random_seed() -> u64 {
    thread_rng().next_u64()
}

pub(crate) fn generate_and_save(name: &str, size: usize, seed: u64) -> Result<(), String> {
    let world = get_generator(size, seed).gen();
    let serialized = bincode::serialize(&world).map_err(|e| { format!("{}", e) })?;
    let mut file = File::create(name).map_err(|e| { format!("{}", e) })?;
    file.write_all(&serialized).map_err(|e| { format!("{}", e) })?;
    Ok(())
}

pub(crate) fn load_world(name: &str) -> Result<robotics_lib::world::world_generator::World, String> {
    let mut file = File::open(name).map_err(|e| { format!("{}", e) })?;
    let mut file_content = Vec::new();

    file.read_to_end(&mut file_content).map_err(|e| { format!("{}", e) })?;

    bincode::deserialize(&file_content).map_err(|e| { format!("{}", e) })?
}