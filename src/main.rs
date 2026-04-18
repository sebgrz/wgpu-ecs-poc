use std::{cell::RefCell, rc::Rc};

use winit::event_loop::EventLoop;

use crate::{renderer::Renderer, window::WindowApplication};

mod renderer;
mod window;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let renderer = Renderer::default();
    let mut app = WindowApplication::init(Rc::new(RefCell::new(renderer)));
    event_loop.run_app(&mut app).unwrap();
}
