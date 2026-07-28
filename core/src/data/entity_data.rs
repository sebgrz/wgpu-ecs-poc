use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct EntityData {
    pub components: Vec<ComponentData>,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ComponentData {
    Player,
    Size {
        width: f32,
        height: f32,
    },
    Position {
        x: f32,
        y: f32,
    },
    SpriteAnimation {
        texture_id: String,
        animation_id: String,
        #[serde(default)]
        is_reversed: bool,
    },
    Tile {
        texture_id: String,
        x: i32,
        y: i32,
        x2: i32,
        y2: i32,
        #[serde(default)]
        is_reversed: bool,
    },
    Animation {
        animation_id: String,
    },
}
