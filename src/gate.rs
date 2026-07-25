use bevy::prelude::*;

use crate::ecs::{Gate, GridLocation, ObstructedSet, SignalSystems};
use crate::map_loader::WorldMap;
use crate::sfx::{PlaySfx, Sfx, SfxSystems};
use crate::signal_logic::{SignalSnapshot, SwitchStates, TimerBank, activation_at};

pub fn gate_plugin(app: &mut App) {
    app.add_systems(
        Update,
        signal_check
            .in_set(SignalSystems::Read)
            .in_set(SfxSystems::Trigger),
    );
}

fn signal_check(
    switches: Res<SwitchStates>,
    world_map: Res<WorldMap>,
    timers: Query<(&GridLocation, &TimerBank)>,
    mut gate_query: Query<(&mut Gate, &mut Transform, &GridLocation)>,
    mut obstructed_set: ResMut<ObstructedSet>,
    mut play_sfx: MessageWriter<PlaySfx>,
) {
    let snapshot = SignalSnapshot::capture(&switches, &timers);
    for (mut gate, mut transform, location) in &mut gate_query {
        let position = uvec2(location.0.x as u32, location.0.z as u32);
        let active = activation_at(&world_map, position, &snapshot).unwrap_or(false);
        let changed = matches!(
            (&*gate, active),
            (Gate::Closed, true) | (Gate::Opened, false)
        );
        if active {
            *gate = Gate::Opened;
            obstructed_set.0.remove(&location.0.as_uvec3());
            transform.translation.y = -5.0;
        } else {
            *gate = Gate::Closed;
            obstructed_set.0.insert(location.0.as_uvec3());
            transform.translation.y = 0.0;
        }
        if changed {
            play_sfx.write(PlaySfx(Sfx::Gate));
        }
    }
}
