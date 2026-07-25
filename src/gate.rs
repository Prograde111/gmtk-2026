use bevy::prelude::*;
use crate::ecs::{Gate, GridLocation, ObstructedSet, SignalAccess, SignalLayers, SignalSystems};
use crate::sfx::{PlaySfx, Sfx, SfxSystems};

pub fn gate_plugin(app: &mut App) {
    app.add_systems(
        Update,
        signal_check
            .in_set(SignalSystems::Read)
            .in_set(SfxSystems::Trigger),
    );
}

fn signal_check(
    signal_layers: Res<SignalLayers>,
    mut gate_query: Query<(&SignalAccess, &mut Gate, &mut Transform, &GridLocation)>,
    mut obstructed_set: ResMut<ObstructedSet>,
    mut play_sfx: MessageWriter<PlaySfx>,
) {
    for (signal_access, mut gate, mut transform, location) in gate_query.iter_mut() {
        if let Some(signal) = signal_layers.0.get(signal_access.0) {
            let changed = matches!((&*gate, *signal), (Gate::Closed, true) | (Gate::Opened, false));
            if *signal {
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
        } else {
            error!("Could not find signal layer from signal access {}", signal_access.0);
        }
    }
}