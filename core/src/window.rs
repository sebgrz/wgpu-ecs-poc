use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::NamedKey,
    window::{Window, WindowId},
};

use crate::{
    input::{KeyType, KeyboardInputAction, SpecialKey},
    renderer::SharedRenderer,
};

pub struct WindowCalls {
    pub create: Box<dyn FnMut()>,
    pub input: Box<dyn FnMut(KeyboardInputAction)>,
    pub update: Box<dyn FnMut(Duration)>,
    pub render: Box<dyn FnMut(Duration)>,
}

pub struct WindowApplication {
    window: Option<Arc<Window>>,
    renderer: SharedRenderer,
    window_calls: WindowCalls,
    last_render_time: Instant,
    last_update_time: Instant,
}

impl WindowApplication {
    pub fn init(renderer: SharedRenderer, window_calls: WindowCalls) -> Self {
        let now = Instant::now();
        Self {
            window: None,
            renderer: renderer,
            window_calls,
            last_render_time: now,
            last_update_time: now,
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
        let window = self.window.clone().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(_size) => {
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let frame_target_time = self.last_render_time + Duration::from_millis(16);
                let update_target_time = self.last_update_time + Duration::from_millis(1);
                let now = Instant::now();
                if now >= update_target_time {
                    (self.window_calls.update)(now - self.last_update_time);
                    self.last_update_time = now;
                }

                if now >= frame_target_time {
                    (self.window_calls.render)(now - self.last_render_time);
                    self.last_render_time = now;
                }
                window.request_redraw();
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic: _,
            } => {
                let key = match event.logical_key {
                    winit::keyboard::Key::Character(ch) => Some(KeyType::CHAR(ch.to_string())),
                    winit::keyboard::Key::Named(named) => match named {
                        NamedKey::Enter => Some(KeyType::SPECIAL(SpecialKey::ENTER)),
                        NamedKey::Escape => Some(KeyType::SPECIAL(SpecialKey::ESCAPE)),
                        NamedKey::ArrowLeft => Some(KeyType::SPECIAL(SpecialKey::LEFT)),
                        NamedKey::ArrowRight => Some(KeyType::SPECIAL(SpecialKey::RIGHT)),
                        NamedKey::ArrowUp => Some(KeyType::SPECIAL(SpecialKey::UP)),
                        NamedKey::ArrowDown => Some(KeyType::SPECIAL(SpecialKey::DOWN)),
                        NamedKey::Space => Some(KeyType::SPECIAL(SpecialKey::SPACE)),
                        _ => None,
                    },
                    _ => None,
                };
                if key.is_none() {
                    return;
                }

                let action = KeyboardInputAction {
                    is_pressed: event.state.is_pressed(),
                    key: key.unwrap(),
                };
                (self.window_calls.input)(action);
            }
            _ => {}
        }
    }
}
