use bevy::prelude::*;
use crate::ecs::{GridLocation, Player, PressurePlate, SignalAccess, SignalLayers, SignalSystems};

pub fn pressure_plate_plugin(app: &mut App) {
    app.add_systems(Update, detect_player.in_set(SignalSystems::Write));
}

pub fn detect_player(
    player: Single<&GridLocation, With<Player>>,
    pressure_plate_query: Query<(&GridLocation, &SignalAccess), (With<PressurePlate>, Without<Player>)>,
    mut signal_layers: ResMut<SignalLayers>,
) {
    let player_location = *player;
    for (pressure_plate_location, signal_access) in pressure_plate_query.iter() {
        if pressure_plate_location.0 == player_location.0 {
            if let Some(signal) = signal_layers.0.get_mut(signal_access.0) {
                *signal |= true;
            } else {
                error!("Could not find signal layer from signal access {}", signal_access.0);
            }
        }
    }
}