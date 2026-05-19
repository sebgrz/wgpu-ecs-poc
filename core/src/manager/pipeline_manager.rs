use std::{borrow::Cow, collections::HashMap, str};

use wgpu::{BindGroupLayout, RenderPipeline};

use crate::{manager::asset_manager::AssetManager, renderer::Renderer};

pub type PipelineManagerError = String;

pub struct PipelineManager<'r> {
    renderer: &'r Renderer,
    pipelines_map: HashMap<String, RenderPipeline>,
}

impl<'r> PipelineManager<'r> {
    pub fn new(renderer: &'r Renderer) -> Self {
        Self {
            renderer,
            pipelines_map: HashMap::new(),
        }
    }

    pub fn create_pipeline(
        &mut self,
        pipeline_id: &str,
        shader_id: &str,
        asset_mgr: &AssetManager,
        bind_group_layouts: Vec<&BindGroupLayout>,
    ) -> Result<(), PipelineManagerError> {
        let optional_bind_group_layouts: Vec<Option<&BindGroupLayout>> =
            bind_group_layouts.into_iter().map(Some).collect();
        let surface_format = self.renderer.borrow_surface_format();
        let (device, _) = self.renderer.borrow_device();
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(format!("{}_pipeline_layout", pipeline_id).as_ref()),
            bind_group_layouts: &optional_bind_group_layouts.as_slice(),
            immediate_size: 0,
        });

        let shader_slice = asset_mgr.load_bytes(shader_id)?;
        let shader_str = str::from_utf8(&shader_slice)
            .map_err(|e| format!("shader_utf8 conv err: {:?}", e).to_string())?;
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader_module"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_str.as_ref())),
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(format!("{}_pipeline", pipeline_id).as_ref()),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                buffers: &[],
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some((*surface_format).into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            multiview_mask: None,
            multisample: wgpu::MultisampleState::default(),
            cache: None,
            depth_stencil: None,
        });

        self.pipelines_map
            .insert(pipeline_id.to_owned(), render_pipeline);
        Ok(())
    }
}
