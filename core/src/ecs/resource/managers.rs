use crate::manager::{
    asset_manager::SharedAssetsManager, file_manager::ShareFileManager,
    pipeline_manager::SharedPipelineManager, texture_manager::SharedTextureManager,
    uniform_buffer_manager::SharedUniformBufferManager,
};

pub struct InnerManagersResource {
    pub assets_manager: SharedAssetsManager,
    pub texture_manager: SharedTextureManager,
    pub uniform_buffer_manager: SharedUniformBufferManager,
    pub pipeline_manager: SharedPipelineManager,
    pub file_manager: ShareFileManager,
}

#[derive(Default)]
pub struct ManagersResource {
    inner: Option<InnerManagersResource>,
}

impl ManagersResource {
    pub fn new(
        assets_manager: SharedAssetsManager,
        texture_manager: SharedTextureManager,
        uniform_buffer_manager: SharedUniformBufferManager,
        pipeline_manager: SharedPipelineManager,
        file_manager: ShareFileManager,
    ) -> Self {
        Self {
            inner: Some(InnerManagersResource {
                assets_manager,
                texture_manager,
                uniform_buffer_manager,
                pipeline_manager,
                file_manager,
            }),
        }
    }

    pub fn get_managers(&self) -> Option<&InnerManagersResource> {
        if let Some(inner) = &self.inner {
            return Some(&inner);
        }

        None
    }
}
