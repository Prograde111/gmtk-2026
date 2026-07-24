use bevy::prelude::*;
use crate::ecs::{CompletedTurn, GlobalPeriodicTimer, SignalAccess, SignalLayers, SignalSystems};

pub fn global_periodic_timer_plugin(app: &mut App) {
    app
        .add_systems(Update, tick_timers.in_set(SignalSystems::Timer));
}

fn tick_timers(
    mut timers: Query<(&mut GlobalPeriodicTimer, &SignalAccess)>,
    mut signal_layers: ResMut<SignalLayers>,
    mut completed_turns: MessageReader<CompletedTurn>,
) {
    for _ in completed_turns.read() {
        for (mut timer, access) in timers.iter_mut() {
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