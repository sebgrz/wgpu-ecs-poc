use std::sync::{Arc, Mutex};

use winit::event_loop::EventLoop;

use wgpu_core::{
    manager::{
        asset_manager::AssetManager, pipeline_manager::PipelineManager,
        texture_manager::TextureManager, uniform_buffer_manager::UniformBufferManager,
    },
    renderer::Renderer,
    window::WindowApplication,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let renderer = Arc::new(Mutex::new(Renderer::default()));
    let asset_manager = AssetManager::new();
    let _buffer_manager = UniformBufferManager::new(renderer.clone());
    let mut texture_manager = TextureManager::new(renderer.clone());
    texture_manager
        .load_texture(&asset_manager, "sample_texture")
        .unwrap();

    let mut pipeline_manager = PipelineManager::new(renderer.clone());
    pipeline_manager
        .create_pipeline(
            "reneder_pipeline_id",
            "sprite_shader",
            &asset_manager,
            vec![],
        )
        .unwrap();

    let mut app = WindowApplication::init(renderer.clone());
    event_loop.run_app(&mut app).unwrap();
}
