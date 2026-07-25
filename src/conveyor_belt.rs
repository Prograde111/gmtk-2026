use crate::ecs::{CompletedTurn, ConveyorBelt, GridLocation, Orientation, Player};
use crate::movement::translate_player;
use crate::sfx::{PlaySfx, Sfx, SfxSystems};
use bevy::prelude::*;

pub fn conveyor_belt_plugin(app: &mut App) {
    app.add_systems(Update, conveyor_belt_move.in_set(SfxSystems::Trigger));
}

pub fn conveyor_belt_move(
    conveyor_belts: Query<(&GridLocation, &Orientation), With<ConveyorBelt>>,
    player: Single<(&mut Transform, &mut GridLocation), (With<Player>, Without<ConveyorBelt>)>,
    mut completed_turns: MessageReader<CompletedTurn>,
    mut play_sfx: MessageWriter<PlaySfx>,
) {
    let (mut player_transform, mut player_location) = player.into_inner();

    for _ in completed_turns.read() {
        for (belt_location, belt_orientation) in conveyor_belts.iter() {
            if belt_location.0 == player_location.0 {
                // player is on belt, time to move them
                translate_player(
                    &mut player_transform,
                    &mut player_location,
                    belt_orientation.0.to_grid_location_offset(),
                    1.0,
                );
                play_sfx.write(PlaySfx(Sfx::Conveyor));
                break;
            }
        }
    }
}
