use bevy::prelude::*;
use crate::ecs::{Altar, AvailableActions, GridLocation, Player, SignalSystems};

pub fn altar_plugin(app: &mut App) {
    app.add_systems(Update, detect_player.in_set(SignalSystems::Write));
}

pub fn detect_player(
    mut player: Single<(&GridLocation, &mut AvailableActions), With<Player>>,
    altar_query: Query<&GridLocation, (With<Altar>, Without<Player>)>,
) {
    let (player_location, mut available_actions) = player.into_inner();
    for altar_location in altar_query.iter() {
        if altar_location.0 == player_location.0 {

        }
    }
}