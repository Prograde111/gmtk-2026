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
    pub signals: MapLayer,
}

pub fn load_world_map(signal_layers: &mut SignalLayers) -> Result<WorldMap, eyre::Error> {
    Ok(WorldMap {
        ground: load_layer(include_bytes!("../assets/maps/ground.png"), "ground")?,
        stuff: load_layer(include_bytes!("../assets/maps/stuff.png"), "stuff")?,
        signals: load_signals_layer(include_bytes!("../assets/maps/signals.png"), "signals", signal_layers)?,
    })
}

fn load_layer(bytes: &[u8], layer: &'static str) -> Result<MapLayer, eyre::Error> {
    let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?
        .into_rgba8();

    check_dimensions(&image, layer)?;

    let mut output = [[0; MAP_HEIGHT]; MAP_WIDTH];
    for (x, y, pixel) in image.enumerate_pixels() {
        output[x as usize][y as usize] = match pixel.0 {
            [_, _, _, 0] => 0, // void
            [255, 255, 255, 255] => 1, // ground
            [255, 0, 0, 255] => 2, // pressure plate
            [154, 114, 46, 255] => 3, // bridge
            [0, 38, 255, 255] => 4, // gate
            rgba => {
                return Err(eyre!("invalid pixel: {x} {y} {rgba:?}"))
            }
        };
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
        /*output[x as usize][y as usize] = match pixel.0 {
            rgba => {
                return Err(eyre!("invalid pixel: {x} {y} {rgba:?}"))
            }
        };*/
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
