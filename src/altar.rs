use crate::ecs::{Altar, AvailableActions, CompletedTurn, GridLocation, ObstructedSet, Player};
use crate::movement::place_player;
use crate::story::{GamePhase, SkillSacrificed, StorySystems};
use bevy::prelude::*;

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
                place_player(
                    &mut player_transform,
                    &mut player_location,
                    completed_turn.old_location.as_vec3(),
                    completed_turn.old_rotation,
                );
                // and now make the altar unable to be entered again
                obstructed_set.0.insert(altar_location.0.as_uvec3());
                altar_transform.translation += vec3(0.0, 1.5, 0.0);
            }
        }
    }
}
