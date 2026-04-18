use winit::event_loop::EventLoop;

use crate::window::WindowApplication;

mod window;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = WindowApplication::init();
    event_loop.run_app(&mut app).unwrap();
}
