use std::collections::HashMap;

#[derive(Default)]
pub struct GameData {
    pub state_data: HashMap<String, StateData>,
    pub sprite_animations: HashMap<String, SpriteAnimationData>
}

pub struct StateData {
    pub texture_id: String,
    pub texture_file_path: String,
}

pub struct SpriteAnimationData {
    pub looping: bool,
    pub keyframes: Vec<SpriteKeyframeData>
}

pub struct SpriteKeyframeData {
    pub tile: [u32; 4],
    pub duration: f32,
}
