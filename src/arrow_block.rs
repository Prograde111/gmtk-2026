use crate::conveyor_belt::conveyor_belt_move;
use crate::ecs::{ArrowBlock, CompletedTurn, GridLocation, Orientation, Player, SignalSystems};
use crate::map_loader::WorldMap;
use crate::movement::face_player;
use crate::signal_logic::{SignalSnapshot, SwitchStates, TimerBank, activation_at};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn arrow_block_plugin(app: &mut App) {
    app.add_systems(Update, rotate_on_signal.in_set(SignalSystems::Read))
        .add_systems(
            Update,
            face_arrow_direction
                .after(rotate_on_signal)
                .after(conveyor_belt_move),
        );
}

fn rotate_on_signal(
    switches: Res<SwitchStates>,
    world_map: Res<WorldMap>,
    timers: Query<(&GridLocation, &TimerBank)>,
    mut completed_turns: MessageReader<CompletedTurn>,
    mut arrow_blocks: Query<(&GridLocation, &mut Orientation, &mut Transform), With<ArrowBlock>>,
) {
    let turns = completed_turns.read().count();
    if turns == 0 {
        return;
    }

    let snapshot = SignalSnapshot::capture(&switches, &timers);
    for (location, mut orientation, mut transform) in &mut arrow_blocks {
        let position = uvec2(location.0.x as u32, location.0.z as u32);
        if !activation_at(&world_map, position, &snapshot).unwrap_or(false) {
            continue;
        }

        for _ in 0..turns {
            orientation.0 = orientation.0.turn_right();
            transform.rotation = Quat::from_rotation_y(-PI / 2.0) * transform.rotation;
        }
    }
}

fn face_arrow_direction(
    mut completed_turns: MessageReader<CompletedTurn>,
    arrow_blocks: Query<(&GridLocation, &Orientation), (With<ArrowBlock>, Without<Player>)>,
    player: Single<
        (&GridLocation, &mut Orientation, &mut Transform),
        (With<Player>, Without<ArrowBlock>),
    >,
) {
    if completed_turns.read().next().is_none() {
        return;
    }

    let (player_location, mut player_orientation, mut player_transform) = player.into_inner();
    let Some((_, arrow_orientation)) = arrow_blocks
        .iter()
        .find(|(location, _)| location.0 == player_location.0)
    else {
        return;
    };

    face_player(
        &mut player_transform,
        &mut player_orientation,
        arrow_orientation.0,
    );
}
