use std::{cell::RefCell, ops::DerefMut, rc::Rc, sync::Arc};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::{core::renderer::Renderer};

pub struct WindowApplication {
    window: Option<Arc<Window>>,
    renderer: Rc<RefCell<Renderer>>,
}

impl WindowApplication {
    pub fn init(renderer: Rc<RefCell<Renderer>>) -> Self {
        Self {
            window: None,
            renderer: renderer,
        }
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes()
            .with_title("window")
            .with_active(true)
            .with_visible(true);

        let window = event_loop.create_window(attributes).unwrap();
        let window_shared = Arc::new(window);

        self.window = Some(window_shared.clone());
        let renderer = self.renderer.clone();
        let mut renderer = renderer.borrow_mut();
        renderer.create_renderer(event_loop.owned_display_handle(), window_shared.clone());

        window_shared.clone().request_redraw();
    }
}

impl ApplicationHandler for WindowApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let mut renderer = self.renderer.borrow_mut();
        let window = self.window.clone().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                renderer.render();
                window.request_redraw();
            }
            _ => {}
        }
    }
}
