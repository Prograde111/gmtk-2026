use bevy::prelude::*;
use crate::ecs::{ActivatedPeriodicTimer, CompletedTurn, SignalAccess, SignalLayers, SignalSystems, TurnCounter};

pub fn activated_periodic_timer_plugin(app: &mut App) {
    app
        .add_systems(Update, activated_tick_timers.in_set(SignalSystems::Timer));
}

fn activated_tick_timers(
    mut timers: Query<(&mut ActivatedPeriodicTimer, &SignalAccess)>,
    mut signal_layers: ResMut<SignalLayers>,
    mut completed_turns: MessageReader<CompletedTurn>,
) {
    for _ in completed_turns.read() {
        for (mut timer, access) in timers.iter_mut() {
            if !timer.is_triggered {
                if let Some(signal) = signal_layers.0.get_mut(access.0) {
                    if *signal {
                        timer.is_triggered = true;
                    } else {
                        continue;
                    }
                } else {
                    error!("Could not find signal layer from signal access {}", access.0);
                    continue;
                }
            }

            timer.turn_tick -= 1;
            if timer.turn_tick == 0 {
                // timer triggered
                timer.turn_tick = timer.period;

                if let Some(signal) = signal_layers.0.get_mut(access.0) {
                    *signal |= true;
                } else {
                    error!("Could not find signal layer from signal access {}", access.0);
                }
            }
        }
    }
}