use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::{entity_data::EntityData, trigger::Trigger, trigger_animation::TriggerAnimation};

#[derive(Deserialize, Serialize, Default)]
pub struct LevelData {
    pub name: String,
    pub entities: Vec<EntityData>,
    pub triggers: HashMap<String, Trigger>,
    pub animations: Vec<TriggerAnimation>,
}
