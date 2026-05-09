use std::{cell::RefCell, rc::Rc};

use winit::event_loop::EventLoop;

use crate::core::{
    manager::{asset_manager::AssetManager, texture_manager::TextureManager},
    renderer::Renderer,
    window::WindowApplication,
};

mod core;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let renderer = Renderer::default();
    let asset_manager = AssetManager::new();
    let mut texture_manager = TextureManager::new(&renderer);
    texture_manager
        .load_texture(&asset_manager, "sample_texture")
        .unwrap();

    let mut app = WindowApplication::init(Rc::new(RefCell::new(renderer)));
    event_loop.run_app(&mut app).unwrap();
}
