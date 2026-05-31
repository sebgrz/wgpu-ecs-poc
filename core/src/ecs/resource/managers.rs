use crate::manager::{
    texture_manager::SharedTextureManager,
    uniform_buffer_manager::{self, SharedUniformBufferManager, UniformBufferManager},
};

struct InnerManagersResource {
    texture_manager: SharedTextureManager,
    uniform_buffer_manager: SharedUniformBufferManager,
}

#[derive(Default)]
pub struct ManagersResource {
    inner: Option<InnerManagersResource>,
}

impl ManagersResource {
    pub fn new(
        texture_manager: SharedTextureManager,
        uniform_buffer_manager: SharedUniformBufferManager,
    ) -> Self {
        Self {
            inner: Some(InnerManagersResource {
                texture_manager,
                uniform_buffer_manager,
            }),
        }
    }

    pub fn get_managers(&self) -> Option<(SharedTextureManager, SharedUniformBufferManager)> {
        if let Some(inner) = &self.inner {
            return Some((
                inner.texture_manager.clone(),
                inner.uniform_buffer_manager.clone(),
            ));
        }

        None
    }
}
