mod game;
mod system;

use winit::event_loop::EventLoop;

use specs::{Builder, DispatcherBuilder, RunNow, WorldExt};
use wgpu_core::{
    ecs::{
        resource::{delta_time::DeltaTimeResource, input::InputResource, state::StateResource},
        system::{
            init::Init, pre_sprite_buffer::PreSpriteBuffer, reload_buffers::ReloadBuffers,
            scene_loader::SceneLoader, sprite_renderer::SpriteRenderer,
        },
    },
    init_managers,
    input::KeyboardInputAction,
    window::{WindowApplication, WindowCalls},
};

use crate::{
    game::state::GameState,
    system::{camera::CameraSystem, menu::MenuSystem, player::PlayerSystem, spawn::SpawnSystem},
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let (renderer, world) = wgpu_core::init();

    let mut dispatcher = DispatcherBuilder::new()
        .with(SpawnSystem, "spawn", &[])
        .with(SceneLoader, "scene_loader", &["spawn"])
        .with(MenuSystem, "menu", &["scene_loader"])
        .with(PlayerSystem, "player", &["scene_loader"])
        .with(CameraSystem, "camera", &["menu", "player"])
        .with(PreSpriteBuffer, "pre_sprite_buffer", &["camera"])
        .with(ReloadBuffers, "reload_buffers", &["camera"])
        .build();

    let world_create = world.clone();
    let renderer_create = renderer.clone();
    let create_call = move || {
        let mut world = world_create.write().unwrap();
        let renderer = renderer_create.clone();

        init_managers(&mut world, renderer);
        let mut state_res = world.write_resource::<StateResource>();
        state_res.game_state = GameState::MENU.to_string();

        let mut init_sys = Init;
        init_sys.run_now(&world);
    };

    let world_update = world.clone();
    let update_call = move |dt| {
        let mut world = world_update.write().unwrap();
        {
            let mut delta_time_res = world.write_resource::<DeltaTimeResource>();
            delta_time_res.time = dt;
        }
        dispatcher.dispatch(&world);
        world.maintain();
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
