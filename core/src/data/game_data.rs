use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::entity_data::EntityData;

#[derive(Deserialize, Serialize, Default)]
pub struct GameData {
    pub state_data: HashMap<String, StateData>,
    pub sprite_animations: HashMap<String, SpriteAnimationData>,
    pub animations: HashMap<String, AnimationData>,
}

#[derive(Deserialize, Serialize)]
pub struct StateData {
    pub texture_id: String,
    pub texture_file_path: String,
    pub entities: Vec<EntityData>,
}

#[derive(Deserialize, Serialize)]
pub struct SpriteAnimationData {
    pub looping: bool,
    pub keyframes: Vec<SpriteKeyframeData>,
}

#[derive(Deserialize, Serialize)]
pub struct SpriteKeyframeData {
    pub tile: [u32; 4],
    pub duration: f32,
}

#[derive(Deserialize, Serialize)]
pub struct AnimationData {
    pub looping: bool,
    pub keyframes: Vec<KeyframeData>,
}

#[derive(Deserialize, Serialize)]
pub struct KeyframeData {
    pub start_pos: [f32; 2],
    pub end_pos: [f32; 2],
    pub duration: f32,
}
