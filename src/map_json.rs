use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use crate::ecs::PlayerAction;

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct MapJson {
    pub elements: Vec<Element>,
    pub altars: Vec<JsonAltar>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Element {
    pub position: [u32; 2],
    pub signal: Signal,
    pub timer: Option<Timer>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Signal {
    pub layer: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Timer {
    pub length: u32,
    pub timer_type: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonAltar {
    pub position: [u32; 2],
    pub action: PlayerAction,
}