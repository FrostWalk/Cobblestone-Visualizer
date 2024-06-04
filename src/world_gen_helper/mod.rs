use std::fs::{File, remove_file};
use std::io::Read;
use std::path::Path;

use oxagworldgenerator::utils::generate_random_seed;
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use oxagworldgenerator::world_generator::presets::content_presets::OxAgContentPresets;
use oxagworldgenerator::world_generator::presets::environmental_presets::OxAgEnvironmentalConditionPresets;
use oxagworldgenerator::world_generator::presets::tile_type_presets::OxAgTileTypePresets;
use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use zstd::stream::{copy_decode, copy_encode};

use crate::config::WalleConfig;

pub(crate) fn get_generator(size: usize, seed: u64) -> Result<OxAgWorldGenerator, String> {
    OxAgWorldGeneratorBuilder::new()
        .set_seed(seed)
        .set_size(size)
        .set_tile_type_options_from_preset(OxAgTileTypePresets::Default)
        .set_content_options_from_preset(OxAgContentPresets::Default)
        .set_environmental_conditions_from_preset(OxAgEnvironmentalConditionPresets::Mixed)
        .build().map_err(|e| format!("{:?}", e))
}

pub(crate) fn generate_and_save(size: usize, seed: u64) -> Result<(), String> {
    let mut world_generator = match get_generator(size, seed) {
        Ok(w) => { w }
        Err(e) => { return Err(e); }
    };

    let path = format!("{}/{}/{}", WalleConfig::static_files_path(), WalleConfig::file_dir(), "world.json");
    world_generator.save(path.as_str()).map_err(|e| format!("{e}"))?;
    let mut file = File::open(path.clone()).map_err(|e| format!("{e}"))?;

    let mut contents = Vec::new();
    file.read_to_end(&mut contents).map_err(|e| format!("{e}"))?;

    let zstdpath = Path::new(WalleConfig::static_files_path().as_str())
        .join(WalleConfig::file_dir().as_str()).join("wall-e_world.zst");

    let mut dest: File = File::create(zstdpath).map_err(|e| format!("{e}"))?;

    copy_encode(contents.as_slice(), &mut dest, 14).map_err(|e| format!("{e}"))?;

    let _ = remove_file(path);

    Ok(())
}

pub(crate) fn load_world() -> Result<OxAgWorldGenerator, String> {
    let zstdpath = Path::new(WalleConfig::static_files_path().as_str())
        .join(WalleConfig::file_dir().as_str()).join("wall-e_world.zst");
    let mut file = File::open(zstdpath.clone()).map_err(|e| { format!("{}", e) })?;

    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content).map_err(|e| { format!("{}", e) })?;

    let mut expanded: Vec<u8> = Vec::with_capacity(file_content.len() * 2);
    copy_decode(file_content.as_slice(), &mut expanded).map_err(|e| format!("{e}"))?;

    let path = format!("{}/{}/{}", WalleConfig::static_files_path(), WalleConfig::file_dir(), "world.json");

    File::create(path.as_str()).map_err(|e| format!("{e}"))?;

    let _ = remove_file(zstdpath);

    OxAgWorldGenerator::builder().load(path.as_str()).map_err(|e| format!("{e}"))
}

#[inline(always)]
pub(crate) fn get_random_seed() -> u64 {
    generate_random_seed()
}
