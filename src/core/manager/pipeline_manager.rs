use std::collections::HashMap;

use wgpu::RenderPipeline;

use crate::core::{manager::texture_manager::TextureManager, renderer::Renderer};

pub(crate) struct PipelineManager<'r, 'tm> {
    renderer: &'r Renderer,
    texture_manager: &'tm TextureManager<'r>,
    pipelines_map: HashMap<String, RenderPipeline>,
}

impl<'r, 'tm> PipelineManager<'r, 'tm> {
    pub(crate) fn new(renderer: &'r Renderer, texture_manager: &'tm TextureManager<'r>) -> Self {
        Self {
            renderer,
            texture_manager,
            pipelines_map: HashMap::new(),
        }
    }
}
