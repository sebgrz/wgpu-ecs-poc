
use winit::event_loop::EventLoop;

use specs::{DispatcherBuilder, RunNow, WorldExt};
use wgpu_core::{
    ecs::system::{
            init::Init, pre_sprite_buffer::PreSpriteBuffer, scene_loader::SceneLoader,
            sprite_renderer::SpriteRenderer,
        },
    init_managers,
    window::{WindowApplication, WindowCalls},
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let (renderer, world) = wgpu_core::init();

    let mut dispatcher = DispatcherBuilder::new()
        .with(SceneLoader, "scene_loader", &[])
        .with(PreSpriteBuffer, "pre_sprite_buffer", &[])
        .build();

    let world_create = world.clone();
    let renderer_create = renderer.clone();
    let create_call = move || {
        let mut world = world_create.write().unwrap();
        let renderer = renderer_create.clone();

        init_managers(&mut world, renderer);

        let mut init_sys = Init;
        init_sys.run_now(&world);
    };

    let world_update = world.clone();
    let update_call = move |_dt| {
        let world = world_update.read().unwrap();
        dispatcher.dispatch(&world);
    };

    let world_render = world.clone();
    let render_call = move |_dt| {
        let world = world_render.read().unwrap();
        let mut sprite_renderer_sys = SpriteRenderer;
        sprite_renderer_sys.run_now(&world);
    };

    let window_calls = WindowCalls {
        create: Box::new(create_call),
        update: Box::new(update_call),
        render: Box::new(render_call),
    };
    let mut app = WindowApplication::init(renderer.clone(), window_calls);
    event_loop.run_app(&mut app).unwrap();
}
