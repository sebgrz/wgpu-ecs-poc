use std::sync::{Arc, RwLock};

use pollster::FutureExt as _;
use wgpu::{
    CurrentSurfaceTexture, Device, DeviceDescriptor, Instance, Queue, TextureFormat,
    TextureViewDescriptor,
};
use winit::{dpi::PhysicalSize, event_loop::OwnedDisplayHandle, window::Window};

pub type SharedRenderer = Arc<RwLock<Renderer>>;

#[derive(Default)]
pub struct Renderer {
    inner_renderer: Option<InnerRenderer>,
}

impl Renderer {
    pub(crate) fn create_renderer(&mut self, display: OwnedDisplayHandle, window: Arc<Window>) {
        let inner_renderer = InnerRenderer::init(display, window).block_on();
        self.inner_renderer = Some(inner_renderer);
    }

    pub(crate) fn borrow_device(&self) -> (&Device, &Queue) {
        let device = &self.inner_renderer.as_ref().unwrap().device;
        let queue = &self.inner_renderer.as_ref().unwrap().queue;

        (device, queue)
    }

    pub(crate) fn borrow_surface_format(&self) -> &TextureFormat {
        &self.inner_renderer.as_ref().unwrap().surface_format
    }

    pub(crate) fn render(&self) {
        if let Some(inner_renderer) = &self.inner_renderer {
            inner_renderer.render();
        }
    }
}

struct InnerRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
}

impl InnerRenderer {
    async fn init(display: OwnedDisplayHandle, window: Arc<Window>) -> Self {
        let instance = Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(Box::new(
            display,
        )));
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .await
            .unwrap();

        let size = window.inner_size();
        let surface = instance.create_surface(window.clone()).unwrap();
        let surface_format = {
            let cap = surface.get_capabilities(&adapter);
            cap.formats[0]
        };

        let renderer = Self {
            device,
            queue,
            surface,
            surface_format,
        };
        renderer.configure_surface(size);

        renderer
    }

    fn configure_surface(&self, size: PhysicalSize<u32>) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    fn render(&self) {
        if let CurrentSurfaceTexture::Success(surface_texture) = self.surface.get_current_texture()
        {
            let texture_view = surface_texture.texture.create_view(&TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

            let mut encoder = self.device.create_command_encoder(&Default::default());
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            // If you wanted to call any drawing commands, they would go here.

            // End the renderpass.
            drop(render_pass);

            self.queue.submit([encoder.finish()]);
            surface_texture.present();
        }
    }
}
