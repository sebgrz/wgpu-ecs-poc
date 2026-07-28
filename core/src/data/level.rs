use serde::{Deserialize, Serialize};

use crate::data::entity_data::EntityData;

#[derive(Deserialize, Serialize, Default)]
pub struct LevelData {
    pub name: String,
    pub entities: Vec<EntityData>,
}
