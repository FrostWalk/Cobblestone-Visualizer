use std::fs::File;
use std::io::Read;
use std::path::Path;

use oxagworldgenerator::world_generator::presets::content_presets::OxAgContentPresets;
use oxagworldgenerator::world_generator::presets::environmental_presets::OxAgEnvironmentalConditionPresets;
use oxagworldgenerator::world_generator::presets::tile_type_presets::OxAgTileTypePresets;
use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use rand::{RngCore, thread_rng};
use robotics_lib::world::world_generator::Generator;
use zstd::stream::copy_encode;

use crate::config::WalleConfig;

pub(crate) fn get_generator(size: usize, seed: u64) -> impl Generator {
    OxAgWorldGeneratorBuilder::new()
        .set_seed(seed)
        .set_size(size)
        .set_tile_type_options_from_preset(OxAgTileTypePresets::Default)
        .set_content_options_from_preset(OxAgContentPresets::Default)
        .set_environmental_conditions_from_preset(OxAgEnvironmentalConditionPresets::Mixed)
        .build()
        .expect("Unable to create a world generator")
}

pub(crate) fn generate_and_save(size: usize, seed: u64) -> Result<(), String> {
    let world = get_generator(size, seed).gen();
    let data = bincode::serialize(&world).map_err(|e| format!("{e}"))?;

    let path = Path::new(WalleConfig::static_files_path().as_str())
        .join(WalleConfig::file_dir().as_str()).join("wall-e_world.zst");
    
    let mut dest: File = File::create(path).map_err(|e| format!("{e}"))?;

    copy_encode(data.as_slice(), &mut dest, 14).map_err(|e| format!("{e}"))?;
    Ok(())
}

pub(crate) fn load_world(name: &str) -> Result<robotics_lib::world::world_generator::World, String> {
    let mut file = File::open(name).map_err(|e| { format!("{}", e) })?;
    let mut file_content = Vec::new();

    file.read_to_end(&mut file_content).map_err(|e| { format!("{}", e) })?;
    bincode::deserialize(&file_content).map_err(|e| { format!("{}", e) })?
}

#[inline(always)]
pub(crate) fn get_random_seed() -> u64 {
    thread_rng().next_u64()
}
