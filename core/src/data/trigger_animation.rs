use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TriggerAnimation {
    pub name: String,
    pub looping: bool,
    pub keyframes: Vec<KeyframeTriggerAnimation>,
}

#[derive(Deserialize, Serialize)]
pub struct KeyframeTriggerAnimation {
    pub action: KeyframeAction,
    pub duration: f32,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum KeyframeAction {
    MoveTo {
        x: f32,
        y: f32,
    },
    MoveToWithSpriteAnimation {
        x: f32,
        y: f32,
        animation_name: String,
    },
}
