use bevy::prelude::*;
use crate::conveyor_belt::conveyor_belt_move;
use crate::ecs::{CompletedTurn, GridLocation, Player, PressurePlate, SignalAccess, SignalLayers, SignalSystems};
use crate::sfx::{PlaySfx, Sfx, SfxSystems};

pub fn pressure_plate_plugin(app: &mut App) {
    app.add_systems(Update, detect_player.in_set(SignalSystems::Write))
        .add_systems(
            Update,
            pressure_plate_sfx
                .in_set(SfxSystems::Trigger)
                .after(conveyor_belt_move),
        );
}

fn pressure_plate_sfx(
    player: Single<&GridLocation, With<Player>>,
    pressure_plates: Query<&GridLocation, (With<PressurePlate>, Without<Player>)>,
    mut completed_turns: MessageReader<CompletedTurn>,
    mut play_sfx: MessageWriter<PlaySfx>,
) {
    let player_location = player.0.as_uvec3();
    for completed_turn in completed_turns.read() {
        if completed_turn.old_location != player_location
            && pressure_plates
                .iter()
                .any(|plate_location| plate_location.0.as_uvec3() == player_location)
        {
            play_sfx.write(PlaySfx(Sfx::PressurePlate));
        }
    }
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