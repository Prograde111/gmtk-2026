use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ecs::PlayerAction;

pub const TIMER_SLOTS: usize = 3;
pub type InitialTimerSlots = [Option<String>; TIMER_SLOTS];

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct MapJson {
    #[serde(default)]
    pub switches: HashMap<String, SwitchTemplate>,
    #[serde(default)]
    pub timers: HashMap<String, TimerTemplate>,
    #[serde(default)]
    pub associations: Vec<MapAssociation>,
    #[serde(default)]
    pub altars: Vec<JsonAltar>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SwitchTemplate {
    pub mode: SwitchMode,
    #[serde(default)]
    pub initial: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SwitchMode {
    Hold,
    Toggle,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum TimerTemplate {
    Periodic {
        period: u32,
        #[serde(default = "one")]
        pulse_turns: u32,
        #[serde(default)]
        initial: bool,
    },
    ActiveAfterCountdown {
        turns: u32,
        #[serde(default)]
        initial: bool,
    },
    ActiveDuringCountdown {
        turns: u32,
        #[serde(default)]
        initial: bool,
    },
}

const fn one() -> u32 {
    1
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MapAssociation {
    pub position: [u32; 2],
    #[serde(default)]
    pub touch_switch: Option<String>,
    #[serde(default)]
    pub timers: Option<InitialTimerSlots>,
    #[serde(default)]
    pub activated_by: Option<SignalExpression>,
    #[serde(default)]
    pub on_activate: Option<InputEffect>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputEffect {
    pub replace_timers: ReplaceTimers,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReplaceTimers {
    pub position: [u32; 2],
    pub with: [TimerReplacement; TIMER_SLOTS],
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum TimerReplacement {
    Template(String),
    Keep(bool),
    Remove(()),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum SignalExpression {
    Constant(bool),
    Switch { switch: String },
    Timer { timer: u8 },
    Not { not: Box<SignalExpression> },
    All { all: Vec<SignalExpression> },
    Any { any: Vec<SignalExpression> },
    Xor { xor: Vec<SignalExpression> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonAltar {
    pub position: [u32; 2],
    pub action: PlayerAction,
}
