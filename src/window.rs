use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    platform::startup_notify::{EventLoopExtStartupNotify, WindowAttributesExtStartupNotify},
    window::{Window, WindowId},
};

pub struct WindowApplication {
    window: Option<Arc<Window>>,
}

impl WindowApplication {
    pub fn init() -> Self {
        Self { window: None }
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes()
            .with_title("window")
            .with_active(true)
            .with_visible(true);

        {
            use winit::platform::startup_notify;
            if let Some(token) = event_loop.read_token_from_env() {
                startup_notify::reset_activation_token_env();
                attributes = attributes.with_activation_token(token);
            }
        }
        let window = event_loop.create_window(attributes).unwrap();
        self.window = Some(Arc::new(window));
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
        let window = self.window.clone().unwrap();

        match event {
            WindowEvent::Resized(size) => {
                // TODO surface
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                window.pre_present_notify();
            }
            WindowEvent::ActivationTokenDone { token: _token, .. } => {
                // #[cfg(any(x11_platform, wayland_platform))]
                {
                    use winit::platform::startup_notify;

                    startup_notify::set_activation_token_env(_token);
                    self.create_window(event_loop);
                }
            }
            _ => {}
        }
    }
}
