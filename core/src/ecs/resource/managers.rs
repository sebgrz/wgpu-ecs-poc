use crate::manager::texture_manager::SharedTextureManager;

#[derive(Default)]
pub struct ManagersResource {
    pub texture_manager: Option<SharedTextureManager>,
}

impl ManagersResource {
    pub fn new(texture_manager: SharedTextureManager) -> Self {
        Self {
            texture_manager: Some(texture_manager),
        }
    }
}
