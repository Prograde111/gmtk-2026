pub mod ecs;
mod game_scene;
pub mod map_loader;
pub mod movement;
pub mod ui;
pub mod pressure_plate;
pub mod gate;
pub mod global_periodic_timer;
pub mod activated_periodic_timer;
pub mod signal_extension_timer;
pub mod on_turn_timer;
pub mod altar_ui;
pub mod altar;
pub mod conveyor_belt;
pub mod music;

use std::collections::HashSet;
use crate::ecs::{CompletedTurn, DebugMode, ObstructedSet, SignalLayers, SignalSystems, TurnCounter};
use crate::game_scene::game_scene_plugin;
use crate::map_loader::load_world_map;
use crate::movement::movement_plugin;
use crate::ui::ui_plugin;
use bevy::prelude::*;
use crate::activated_periodic_timer::activated_periodic_timer_plugin;
use crate::conveyor_belt::conveyor_belt_plugin;
use crate::gate::gate_plugin;
use crate::global_periodic_timer::global_periodic_timer_plugin;
use crate::on_turn_timer::on_turn_timer_plugin;
use crate::pressure_plate::pressure_plate_plugin;
use crate::signal_extension_timer::signal_extension_timer_plugin;
use crate::music::music_plugin;

pub const MAX_TURN_COUNT: u32 = 1000;

pub const PLAYER_SIZE: Vec3 = vec3(1.0, 1.0, 1.0);
pub const GRID_SIZE: Vec2 = vec2(1.0, 1.0);

// in seconds
pub const ANIMATION_LENGTH: f32 = 0.25;

fn main() {
    let mut signal_layers = SignalLayers(vec![false]);
    let world_map = load_world_map(&mut signal_layers).expect("failed to load world map");
    let debug_mode = std::env::args().any(|argument| argument == "--debug");

    println!("Generated signal layers: {:?}", signal_layers);

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DebugMode(debug_mode))
        .insert_resource(signal_layers)
        .insert_resource(world_map)
        .insert_resource(TurnCounter(MAX_TURN_COUNT))
        .insert_resource(ObstructedSet(HashSet::new()))
        //.insert_resource(SpecialTileSet(HashMap::new()))
        .add_message::<CompletedTurn>()
        .add_plugins(game_scene_plugin)
        .add_plugins(movement_plugin)
        .add_plugins(ui_plugin)
        .add_plugins(pressure_plate_plugin)
        .add_plugins(gate_plugin)
        .add_plugins(conveyor_belt_plugin)
        .add_plugins(global_periodic_timer_plugin)
        .add_plugins(activated_periodic_timer_plugin)
        .add_plugins(signal_extension_timer_plugin)
        .add_plugins(on_turn_timer_plugin)
        .add_plugins(music_plugin)
        .add_systems(Update, clear_signal_layers.in_set(SignalSystems::Clear).after(movement::do_movement))
        .configure_sets(Update, (SignalSystems::Clear, SignalSystems::Write, SignalSystems::Timer, SignalSystems::Read).chain())
        .run();
}

fn clear_signal_layers(
    mut signal_layers: ResMut<SignalLayers>,
    mut completed_turns: MessageReader<CompletedTurn>,
) {
    for _ in completed_turns.read() {
        for signal in &mut signal_layers.0 {
            *signal = false;
        }
    }
}