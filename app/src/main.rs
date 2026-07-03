use winit::event_loop::EventLoop;

use specs::{Builder, DispatcherBuilder, RunNow, WorldExt};
use wgpu_core::{
    ecs::{
        component::{position::Position, tile::Tile},
        resource::input::InputResource,
        system::{
            init::Init, pre_sprite_buffer::PreSpriteBuffer, scene_loader::SceneLoader,
            sprite_renderer::SpriteRenderer,
        },
    },
    init_managers,
    input::KeyboardInputAction,
    window::{WindowApplication, WindowCalls},
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let (renderer, world) = wgpu_core::init();
    {
        let mut world = world.write().unwrap();
        world
            .create_entity()
            .with(Position { x: 10, y: 20 })
            .with(Tile {
                texture_id: "sprites_texture".to_owned(),
                x: 3,
                y: 16,
                x2: 15,
                y2: 32,
            })
            .build();
        world
            .create_entity()
            .with(Position { x: 200, y: 125 })
            .with(Tile {
                texture_id: "sprites_texture".to_owned(),
                x: 19,
                y: 5,
                x2: 27,
                y2: 16,
            })
            .build();
    }

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

    let world_input = world.clone();
    let input_call = move |key_action: KeyboardInputAction| {
        let world = world_input.read().unwrap();
        let mut input_res = world.write_resource::<InputResource>();
        if let Some(key_value) = input_res.keys.get(&key_action.key) {
            if *key_value == key_action.is_pressed {
                return;
            }
        }
        input_res
            .keys
            .insert(key_action.key.clone(), key_action.is_pressed);
        println!("key: {:?}", key_action);
    };

    let window_calls = WindowCalls {
        create: Box::new(create_call),
        input: Box::new(input_call),
        update: Box::new(update_call),
        render: Box::new(render_call),
    };
    let mut app = WindowApplication::init(renderer.clone(), window_calls);
    event_loop.run_app(&mut app).unwrap();
}
