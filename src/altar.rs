use bevy::prelude::*;
use crate::ecs::{Altar, AvailableActions, CompletedTurn, GridLocation, ObstructedSet, Player};
use crate::story::{GamePhase, SkillSacrificed, StorySystems};
use crate::{GRID_SIZE, PLAYER_SIZE};

pub fn altar_plugin(app: &mut App) {
    app.add_systems(
        Update,
        detect_player
            .run_if(in_state(GamePhase::Playing))
            .before(StorySystems::Events),
    );
}

pub fn detect_player(
    player: Single<(&mut GridLocation, &mut AvailableActions, &mut Transform), With<Player>>,
    mut altar_query: Query<(&GridLocation, &Altar, &mut Transform), Without<Player>>,
    mut obstructed_set: ResMut<ObstructedSet>,
    mut completed_turns: MessageReader<CompletedTurn>,
    mut sacrifices: MessageWriter<SkillSacrificed>,
) {
    let (mut player_location, mut available_actions, mut player_transform) = player.into_inner();
    for completed_turn in completed_turns.read() {
        for (altar_location, altar, mut altar_transform) in altar_query.iter_mut() {
            if altar_location.0 == player_location.0 {
                // we're on the altar, time to delete an action
                if !available_actions.remove(altar.0) {
                    continue;
                }
                sacrifices.write(SkillSacrificed(altar.0));
                info!("Removed action {:?}", altar.0.key_code());
                // now move back
                player_location.0 = completed_turn.old_location.as_vec3();
                player_transform.translation = completed_turn.old_location.as_vec3() * vec3(GRID_SIZE.x, 0.0, GRID_SIZE.y) + vec3(0.0, PLAYER_SIZE.y/2.0, 0.0);
                player_transform.rotation = completed_turn.old_rotation;
                // and now make the altar unable to be entered again
                obstructed_set.0.insert(altar_location.0.as_uvec3());
                altar_transform.translation += vec3(0.0, 1.5, 0.0);
            }
        }
    }
}