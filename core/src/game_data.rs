use std::collections::HashMap;

#[derive(Default)]
pub struct GameData {
    pub state_data: HashMap<String, StateData>,
    pub sprite_animations: HashMap<String, SpriteAnimationData>,
    pub animations: HashMap<String, AnimationData>,
}

pub struct StateData {
    pub texture_id: String,
    pub texture_file_path: String,
}

pub struct SpriteAnimationData {
    pub looping: bool,
    pub keyframes: Vec<SpriteKeyframeData>,
}

pub struct SpriteKeyframeData {
    pub tile: [u32; 4],
    pub duration: f32,
}

pub struct AnimationData {
    pub looping: bool,
    pub keyframes: Vec<KeyframeData>,
}

pub struct KeyframeData {
    pub start_pos: [f32; 2],
    pub end_pos: [f32; 2],
    pub duration: f32,
}
