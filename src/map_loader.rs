use std::collections::HashMap;
use bevy::prelude::Resource;
use eyre::eyre;
use image::{Pixel, RgbaImage};
use crate::ecs::SignalLayers;

pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;
pub type MapLayer = [[u32; MAP_HEIGHT]; MAP_WIDTH];

#[derive(Resource)]
pub struct WorldMap {
    pub ground: MapLayer,
    pub stuff: MapLayer,
    pub stuff_orientation: MapLayer,
    pub signals: MapLayer,
    pub timer: MapLayer,
}

pub fn load_world_map(signal_layers: &mut SignalLayers) -> Result<WorldMap, eyre::Error> {
    let mut regular_color_mapping = HashMap::new();
    regular_color_mapping.insert([255, 255, 255, 255], 1); // ground
    regular_color_mapping.insert([255, 0, 0, 255], 2); // pressure plate
    regular_color_mapping.insert([154, 114, 46, 255], 3); // bridge
    regular_color_mapping.insert([0, 38, 255, 255], 4); // gate

    let mut orientation_mapping = HashMap::new();
    orientation_mapping.insert([0, 255, 255, 255], 1); // north
    orientation_mapping.insert([255, 255, 0, 255], 2); // west
    orientation_mapping.insert([255, 0, 255, 255], 3); // south
    orientation_mapping.insert([0, 0, 0, 255], 4); // east
    Ok(WorldMap {
        ground: load_layer(include_bytes!("../assets/maps/ground.png"), "ground", regular_color_mapping.clone())?,
        stuff: load_layer(include_bytes!("../assets/maps/stuff.png"), "stuff", regular_color_mapping)?,
        stuff_orientation: load_layer(include_bytes!("../assets/maps/stuff_orientation.png"), "stuff_orientation", orientation_mapping)?,
        signals: load_signals_layer(include_bytes!("../assets/maps/signals.png"), "signals", signal_layers)?,
        timer: load_timer_layer(include_bytes!("../assets/maps/timer.png"), "timer")?,
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
fn load_signals_layer(bytes: &[u8], layer: &'static str, signal_layers: &mut SignalLayers) -> Result<MapLayer, eyre::Error> {
    let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?
        .into_rgba8();
    check_dimensions(&image, layer)?;
    let mut output = [[0; MAP_HEIGHT]; MAP_WIDTH];
    let mut current_signal: u32 = 1;
    let mut signal_color_map = HashMap::new();
    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel.alpha() == 0 { continue }
        if let Some(signal_id) = signal_color_map.get(pixel) {
            output[x as usize][y as usize] = *signal_id;
        } else {
            signal_layers.0.push(false);
            signal_color_map.insert(pixel, current_signal);
            output[x as usize][y as usize] = current_signal;
            current_signal += 1;
        }
    }

    Ok(output)
}
fn load_timer_layer(bytes: &[u8], layer: &'static str) -> Result<MapLayer, eyre::Error> {
    let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?
        .into_rgba8();
    check_dimensions(&image, layer)?;
    let mut output = [[0; MAP_HEIGHT]; MAP_WIDTH];

    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel.alpha() == 0 { continue }
        let blue = pixel.0[2] as u32;
        let timer_type = match blue {
            100 => 1, // signal extension
            150 => 2, // periodic after activation
            255 => 3, // global periodic
            _ => 0
        };
        let length = pixel.0[0] as u32;
        output[x as usize][y as usize] = timer_type << 8 | length;
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
