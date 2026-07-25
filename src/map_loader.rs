use std::collections::{HashMap, HashSet};
use bevy::math::{uvec2, UVec2};
use bevy::prelude::Resource;
use eyre::eyre;
use image::RgbaImage;
use crate::ecs::{Altar, SignalLayers};
use crate::map_json::{MapJson, Signal, Timer};

pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;
pub type MapLayer = [[u32; MAP_HEIGHT]; MAP_WIDTH];

#[derive(Resource)]
pub struct WorldMap {
    pub ground: MapLayer,
    pub stuff: MapLayer,
    pub stuff_orientation: MapLayer,
    pub signals: HashMap<UVec2, Signal>,
    pub timers: HashMap<UVec2, Timer>,
    pub altars: HashMap<UVec2, Altar>,
}

pub fn load_world_map(map_json: &MapJson, signal_layers: &mut SignalLayers) -> Result<WorldMap, eyre::Error> {
    let mut regular_color_mapping = HashMap::new();
    regular_color_mapping.insert([255, 255, 255, 255], 1); // ground
    regular_color_mapping.insert([255, 0, 0, 255], 2); // pressure plate
    regular_color_mapping.insert([154, 114, 46, 255], 3); // bridge
    regular_color_mapping.insert([0, 38, 255, 255], 4); // gate
    regular_color_mapping.insert([99, 99, 99, 255], 5); // conveyor belt

    let mut orientation_mapping = HashMap::new();
    orientation_mapping.insert([0, 255, 255, 255], 1); // north
    orientation_mapping.insert([255, 255, 0, 255], 2); // west
    orientation_mapping.insert([255, 0, 255, 255], 3); // south
    orientation_mapping.insert([0, 0, 0, 255], 4); // east

    let mut signals = HashMap::new();
    let mut timers = HashMap::new();
    let mut current_signals = HashSet::new();
    for element in map_json.elements.clone() {
        if current_signals.get(&element.signal.layer).is_none() {
            current_signals.insert(element.signal.layer);
            signal_layers.0.push(false);
        }
        signals.insert(uvec2(element.position[0], element.position[1]), element.signal);

        if let Some(timer) = element.timer {
            timers.insert(uvec2(element.position[0], element.position[1]), timer);
        }
    }
    let mut altars = HashMap::new();
    for altar in map_json.altars.clone() {
        altars.insert(uvec2(altar.position[0], altar.position[1]), Altar(altar.action));
    }
    Ok(WorldMap {
        ground: load_layer(include_bytes!("../assets/maps/ground.png"), "ground", regular_color_mapping.clone())?,
        stuff: load_layer(include_bytes!("../assets/maps/stuff.png"), "stuff", regular_color_mapping)?,
        stuff_orientation: load_layer(include_bytes!("../assets/maps/stuff_orientation.png"), "stuff_orientation", orientation_mapping)?,
        signals,
        timers,
        altars,
    })
}

fn load_layer(bytes: &[u8], layer: &'static str, color_mapping: HashMap<[u8; 4], u32>) -> Result<MapLayer, eyre::Error> {
    let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?
        .into_rgba8();

    check_dimensions(&image, layer)?;

    let mut output = [[0; MAP_HEIGHT]; MAP_WIDTH];
    for (x, y, pixel) in image.enumerate_pixels() {
        if let Some(mapping) = color_mapping.get(&pixel.0) {
            output[x as usize][y as usize] = *mapping;
        } else {
            if pixel.0[3] == 0 {
                output[x as usize][y as usize] = 0;
            } else {
                return Err(eyre!("invalid pixel: {x} {y} {:?}", pixel.0))
            }
        }
    }
    Ok(output)
}

fn check_dimensions(image: &RgbaImage, layer: &'static str) -> Result<(), eyre::Error> {
    let (width, height) = image.dimensions();
    if width != MAP_WIDTH as u32 || height != MAP_HEIGHT as u32 {
        return Err(eyre!("wrong size: {layer} (should be {MAP_WIDTH} {MAP_HEIGHT}; got {width} {height})"));
    }
    Ok(())
}
