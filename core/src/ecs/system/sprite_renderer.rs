use specs::{Read, System};
use wgpu::{CurrentSurfaceTexture, TextureViewDescriptor};

use crate::{
    ecs::{
        resource::{
            buffers::BuffersResource,
            managers::ManagersResource,
            renderer::RendererResource,
            state::{State, StateResource},
        },
        CAMERA_BUFFER_UNIFORM, MENU_TEXTURE_ID, SPRITES_BUFFER_UNIFORM, SPRITES_RENDER_PIPELINE_ID,
        SPRITES_TEXTURE_ID,
    },
    manager::texture_manager::TextureObject,
};

pub struct SpriteRenderer;

impl<'a> System<'a> for SpriteRenderer {
    type SystemData = (
        Read<'a, ManagersResource>,
        Read<'a, RendererResource>,
        Read<'a, BuffersResource>,
        Read<'a, StateResource>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (managers_resource, renderer_resource, sprites_buffer_resources, state_res) = data;

        if state_res.state != State::RENDER {
            return;
        }

        let inner_mangers = managers_resource.get_managers().unwrap();
        let uniform_buffer_manager = inner_mangers.uniform_buffer_manager.read().unwrap();
        let pipeline_manager = inner_mangers.pipeline_manager.read().unwrap();
        let tex_manager = inner_mangers.texture_manager.read().unwrap();

        // bind groups
        let (sprites_bind_group, _) = uniform_buffer_manager
            .borrow_bind_group(SPRITES_BUFFER_UNIFORM)
            .unwrap();
        let (camera_bind_group, _) = uniform_buffer_manager
            .borrow_bind_group(CAMERA_BUFFER_UNIFORM)
            .unwrap();

        // texture
        // TODO: get textures from level_manager
        let texture_obj: &TextureObject = match state_res.game_state.as_str() {
            "MENU" => Some(tex_manager.borrow_object(MENU_TEXTURE_ID)),
            "LEVEL" => Some(tex_manager.borrow_object(SPRITES_TEXTURE_ID)),
            _ => None,
        }
        .unwrap();

        // prepare pipeline

        if let Some(render_pipeline) = pipeline_manager.borrow_pipeline(SPRITES_RENDER_PIPELINE_ID)
        {
            if let Some(arc_renderer) = &renderer_resource.renderer {
                let renderer = arc_renderer.read().unwrap();
                let (device, queue) = renderer.borrow_device();
                let surface = renderer.borrow_surface();
                let surface_format = renderer.borrow_surface_format();
                if let CurrentSurfaceTexture::Success(surface_texture) =
                    surface.get_current_texture()
                {
                    let texture_view =
                        surface_texture.texture.create_view(&TextureViewDescriptor {
                            format: Some(surface_format.add_srgb_suffix()),
                            ..Default::default()
                        });

                    let mut encoder = device.create_command_encoder(&Default::default());
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

                    render_pass.set_pipeline(render_pipeline);
                    render_pass.set_bind_group(0, Some(&texture_obj.bind_group), &[]);
                    render_pass.set_bind_group(1, Some(sprites_bind_group), &[]);
                    render_pass.set_bind_group(2, Some(camera_bind_group), &[]);

                    render_pass.draw(0..6, 0..sprites_buffer_resources.sprites_size as u32);
                    // End the renderpass.
                    drop(render_pass);

                    queue.submit([encoder.finish()]);
                    surface_texture.present();
                }
            }
        }
    }
}
