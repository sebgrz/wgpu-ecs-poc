use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::renderer::SharedRenderer;

pub struct WindowCalls {
    pub create: Box<dyn FnMut()>,
    pub render: Box<dyn FnMut(Duration)>,
}

pub struct WindowApplication {
    window: Option<Arc<Window>>,
    renderer: SharedRenderer,
    window_calls: WindowCalls,
}

impl WindowApplication {
    pub fn init(renderer: SharedRenderer, window_calls: WindowCalls) -> Self {
        Self {
            window: None,
            renderer: renderer,
            window_calls,
        }
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title("window")
            .with_active(true)
            .with_visible(true);

        let window = event_loop.create_window(attributes).unwrap();
        let window_shared = Arc::new(window);

        self.window = Some(window_shared.clone());
        {
            let mut renderer = self.renderer.write().unwrap();
            renderer.create_renderer(event_loop.owned_display_handle(), window_shared.clone());
        }

        (self.window_calls.create)();

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
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // let renderer = self.renderer.read().unwrap();
        let window = self.window.clone().unwrap();
        let mut last_time = Instant::now();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(_size) => {
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = now - last_time;
                // renderer.render(); TODO is not required, render is doing from ecs
                (self.window_calls.render)(dt);
                window.request_redraw();
                last_time = now;
            }
            _ => {}
        }
    }
}
