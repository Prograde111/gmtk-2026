use bevy::prelude::*;
use crate::ecs::{CompletedTurn, SignalAccess, SignalExtensionTimer, SignalLayers, SignalSystems};

pub fn signal_extension_timer_plugin(app: &mut App) {
    app
        .add_systems(Update, tick_timers.in_set(SignalSystems::Timer));
}

fn tick_timers(
    mut timers: Query<(&mut SignalExtensionTimer, &SignalAccess)>,
    mut signal_layers: ResMut<SignalLayers>,
    mut completed_turns: MessageReader<CompletedTurn>,
) {
    for _ in completed_turns.read() {
        for (mut timer, access) in timers.iter_mut() {
            if !timer.is_triggered {
                if let Some(signal) = signal_layers.0.get_mut(access.0) {
                    if *signal {
                        timer.turn_tick = timer.length;
                        timer.is_triggered = true;
                    } else {
                        continue;
                    }
                } else {
                    error!("Could not find signal layer from signal access {}", access.0);
                    continue;
                }
            }

            if let Some(signal) = signal_layers.0.get_mut(access.0) {
                *signal |= true;
            } else {
                error!("Could not find signal layer from signal access {}", access.0);
            }
            timer.turn_tick -= 1;
            if timer.turn_tick == 0 {
                // timer triggered
                timer.is_triggered = false;
            }
        }
    }
}