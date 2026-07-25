use crate::ecs::Altar;
use crate::map_json::{
    InitialTimerSlots, MapJson, ReplaceTimers, SignalExpression, TimerReplacement, TimerTemplate,
};
use bevy::math::{UVec2, uvec2};
use bevy::prelude::Resource;
use eyre::{bail, eyre};
use image::RgbaImage;
use std::collections::{HashMap, HashSet};

pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;
pub type MapLayer = [[u32; MAP_HEIGHT]; MAP_WIDTH];

#[derive(Resource)]
pub struct WorldMap {
    pub ground: MapLayer,
    pub stuff: MapLayer,
    pub stuff_orientation: MapLayer,
    pub touched_switches: HashMap<UVec2, Vec<String>>,
    pub timer_banks: HashMap<UVec2, InitialTimerSlots>,
    pub activation_conditions: HashMap<UVec2, Vec<SignalExpression>>,
    pub input_effects: HashMap<UVec2, Vec<ReplaceTimers>>,
    pub timers: HashMap<String, TimerTemplate>,
    pub altars: HashMap<UVec2, Altar>,
}

pub fn load_world_map(map_json: &MapJson) -> Result<WorldMap, eyre::Error> {
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

    let mut touched_switches: HashMap<UVec2, Vec<String>> = HashMap::new();
    let mut timer_banks = HashMap::new();
    let mut configured_timer_banks = HashSet::new();
    let mut activation_conditions: HashMap<UVec2, Vec<SignalExpression>> = HashMap::new();
    let mut input_effects: HashMap<UVec2, Vec<ReplaceTimers>> = HashMap::new();
    for association in &map_json.associations {
        let position = association.position.into();

        if let Some(switch) = &association.touch_switch {
            touched_switches
                .entry(position)
                .or_default()
                .push(switch.clone());
        }

        if let Some(slots) = &association.timers {

            configured_timer_banks.insert(position);
            timer_banks.insert(position, slots.clone());
        }

        if let Some(expression) = &association.activated_by {
            activation_conditions
                .entry(position)
                .or_default()
                .push(expression.clone());
        }

        if let Some(effect) = &association.on_activate {
            let target = effect.replace_timers.position.into();
            timer_banks
                .entry(target)
                .or_insert_with(|| [None, None, None]);
            input_effects
                .entry(position)
                .or_default()
                .push(effect.replace_timers.clone());
        }
    }

    let mut altars = HashMap::new();
    for altar in map_json.altars.clone() {
        altars.insert(
            uvec2(altar.position[0], altar.position[1]),
            Altar(altar.action),
        );
    }

    Ok(WorldMap {
        ground: load_layer(
            include_bytes!("../assets/maps/ground.png"),
            "ground",
            regular_color_mapping.clone(),
        )?,
        stuff: load_layer(
            include_bytes!("../assets/maps/stuff.png"),
            "stuff",
            regular_color_mapping,
        )?,
        stuff_orientation: load_layer(
            include_bytes!("../assets/maps/stuff_orientation.png"),
            "stuff_orientation",
            orientation_mapping,
        )?,
        touched_switches,
        timer_banks,
        activation_conditions,
        input_effects,
        timers: map_json.timers.clone(),
        altars,
    })
}


fn load_layer(
    bytes: &[u8],
    layer: &'static str,
    color_mapping: HashMap<[u8; 4], u32>,
) -> Result<MapLayer, eyre::Error> {
    let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?.into_rgba8();

    check_dimensions(&image, layer)?;

    let mut output = [[0; MAP_HEIGHT]; MAP_WIDTH];
    for (x, y, pixel) in image.enumerate_pixels() {
        if let Some(mapping) = color_mapping.get(&pixel.0) {
            output[x as usize][y as usize] = *mapping;
        } else {
            if pixel.0[3] == 0 {
                output[x as usize][y as usize] = 0;
            } else {
                return Err(eyre!("invalid pixel: {x} {y} {:?}", pixel.0));
            }
        }
    }
    Ok(output)
}

fn check_dimensions(image: &RgbaImage, layer: &'static str) -> Result<(), eyre::Error> {
    let (width, height) = image.dimensions();
    if width != MAP_WIDTH as u32 || height != MAP_HEIGHT as u32 {
        return Err(eyre!(
            "wrong size: {layer} (should be {MAP_WIDTH} {MAP_HEIGHT}; got {width} {height})"
        ));
    }
    Ok(())
}
