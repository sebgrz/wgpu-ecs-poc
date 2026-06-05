use crate::manager::{
    pipeline_manager::SharedPipelineManager,
    texture_manager::SharedTextureManager,
    uniform_buffer_manager::{self, SharedUniformBufferManager, UniformBufferManager},
};

struct InnerManagersResource {
    texture_manager: SharedTextureManager,
    uniform_buffer_manager: SharedUniformBufferManager,
    pipeline_manager: SharedPipelineManager,
}

#[derive(Default)]
pub struct ManagersResource {
    inner: Option<InnerManagersResource>,
}

impl ManagersResource {
    pub fn new(
        texture_manager: SharedTextureManager,
        uniform_buffer_manager: SharedUniformBufferManager,
        pipeline_manager: SharedPipelineManager,
    ) -> Self {
        Self {
            inner: Some(InnerManagersResource {
                texture_manager,
                uniform_buffer_manager,
                pipeline_manager,
            }),
        }
    }

    pub fn get_managers(
        &self,
    ) -> Option<(
        SharedTextureManager,
        SharedUniformBufferManager,
        SharedPipelineManager,
    )> {
        if let Some(inner) = &self.inner {
            return Some((
                inner.texture_manager.clone(),
                inner.uniform_buffer_manager.clone(),
                inner.pipeline_manager.clone(),
            ));
        }

        None
    }
}
