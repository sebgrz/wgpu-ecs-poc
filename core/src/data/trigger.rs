use serde::{Deserialize, Serialize};

use crate::input::KeyType;

#[derive(Deserialize, Serialize)]
pub struct Trigger {
    pub actions: Vec<TriggerAction>,
    pub run: Vec<TriggerRun>,
    pub once: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum TriggerAction {
    Box { x: f32, y: f32, x1: f32, y1: f32 },
    KeyDown { key_type: KeyType },
    Marker { name: String },
    // TODO: Tap, Click ...
}
#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum TriggerRun {
    Animation { name: String },
}
