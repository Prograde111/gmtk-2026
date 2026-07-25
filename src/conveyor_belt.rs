use bevy::prelude::*;
use crate::ecs::{CompletedTurn, ConveyorBelt, GridLocation, Orientation, Player};
use crate::GRID_SIZE;

pub fn conveyor_belt_plugin(app: &mut App) {
    app.add_systems(Update, conveyor_belt_move);
}

pub fn conveyor_belt_move(
    conveyor_belts: Query<(&GridLocation, &Orientation), With<ConveyorBelt>>,
    mut player: Single<(&mut Transform, &mut GridLocation), (With<Player>, Without<ConveyorBelt>)>,
    mut completed_turns: MessageReader<CompletedTurn>,
) {
    let (mut player_transform, mut player_location) = player.into_inner();

    for _ in completed_turns.read() {
        for (belt_location, belt_orientation) in conveyor_belts.iter() {
            if belt_location.0 == player_location.0 {
                // player is on belt, time to move them
                let translation_vector = belt_orientation.0.to_vec_direction() * vec3(GRID_SIZE.x, 0.0, GRID_SIZE.y);
                player_transform.translation += translation_vector;
                player_location.0 += belt_orientation.0.to_grid_location_offset();
                break;
            }
        }
    }
}