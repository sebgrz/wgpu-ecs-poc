use std::collections::HashMap;

use crate::input::KeyType;

#[derive(Default)]
pub struct InputResource {
    pub keys: HashMap<KeyType, bool>,
}
