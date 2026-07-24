use bevy::prelude::*;
use crate::ecs::{AfterTurnTimer, CompletedTurn, SignalAccess, SignalExtensionTimer, SignalLayers, SignalSystems, UntilTurnTimer};

pub fn on_turn_timer_plugin(app: &mut App) {
    app
        .add_systems(Update, tick_until_turn_timers.in_set(SignalSystems::Timer))
        .add_systems(Update, tick_after_turn_timers.in_set(SignalSystems::Timer));
}

fn tick_until_turn_timers(
    mut timers: Query<(&mut UntilTurnTimer, &SignalAccess)>,
    mut signal_layers: ResMut<SignalLayers>,
    mut completed_turns: MessageReader<CompletedTurn>,
) {
    for _ in completed_turns.read() {
        for (mut timer, access) in timers.iter_mut() {
            timer.turn_tick += 1;
            if timer.turn_tick <= timer.trigger_turn {
                if let Some(signal) = signal_layers.0.get_mut(access.0) {
                    *signal |= true;
                } else {
                    error!("Could not find signal layer from signal access {}", access.0);
                }
            }
        }
    }
}
fn tick_after_turn_timers(
    mut timers: Query<(&mut AfterTurnTimer, &SignalAccess)>,
    mut signal_layers: ResMut<SignalLayers>,
    mut completed_turns: MessageReader<CompletedTurn>,
) {
    for _ in completed_turns.read() {
        for (mut timer, access) in timers.iter_mut() {
            timer.turn_tick += 1;
            if timer.turn_tick >= timer.trigger_turn {
                if let Some(signal) = signal_layers.0.get_mut(access.0) {
                    *signal |= true;
                } else {
                    error!("Could not find signal layer from signal access {}", access.0);
                }
            }
        }
    }
}