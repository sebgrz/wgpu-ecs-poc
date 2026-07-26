use std::collections::HashMap;

#[derive(Default)]
pub struct GameData {
    pub state_data: HashMap<String, StateData>,
}

pub struct StateData {
    pub texture_id: String,
    pub texture_file_path: String,
}
