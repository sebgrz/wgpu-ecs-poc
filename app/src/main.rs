
use winit::event_loop::EventLoop;

use wgpu_core::window::WindowApplication;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let renderer = wgpu_core::init();
    let mut app = WindowApplication::init(renderer.clone());
    event_loop.run_app(&mut app).unwrap();
}
