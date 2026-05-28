use std::rc::Rc;

use crate::manager::texture_manager::{SharedTextureManager, TextureManager};

#[derive(Default)]
pub struct ManagersResource {
    texture_manager: Option<SharedTextureManager>,
}
