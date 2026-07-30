use glam::Mat4;
use specs::{Read, System, Write};

use crate::{
    ecs::{
        resource::{
            game::GameResource,
            managers::ManagersResource,
            state::{State, StateResource},
        },
        BIND_GROUP_MAIN_BUFFERS, CAMERA_BUFFER_UNIFORM, MAIN_SHADERS_ID, SPRITES_BUFFER_UNIFORM,
        SPRITES_RENDER_PIPELINE_ID,
    },
    manager::uniform_buffer_manager::UniformBufferEntry,
    uniform::{sprite::Sprite, UniformBufSize},
};

pub struct SceneLoader;

impl<'a> System<'a> for SceneLoader {
    type SystemData = (
        Read<'a, GameResource>,
        Write<'a, ManagersResource>,
        Write<'a, StateResource>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (game_res, managers_res, mut state_res) = data;
        if state_res.state != State::SCENE {
            return;
        }

        let inner_managers = managers_res.get_managers().unwrap();
        let assets_manager = inner_managers.assets_manager.read().unwrap();

        // load textures
        let mut tex_manager = inner_managers.texture_manager.write().unwrap();
        tex_manager.unload_all().unwrap();

        let tex_id = game_res.data.state_data[&state_res.game_state]
            .texture_id
            .clone();
        tex_manager.load_texture(&assets_manager, &tex_id).unwrap();

        // prepare uniforms
        let mut uniform_buffer_manager = inner_managers.uniform_buffer_manager.write().unwrap();
        uniform_buffer_manager.cleanup_all();

        let entries = vec![
            UniformBufferEntry {
                size_fn: Sprite::size_fn(),
                buffer_name: SPRITES_BUFFER_UNIFORM.to_string(),
                binding: 0,
                items_count: 1024,
            },
            UniformBufferEntry {
                size_fn: Mat4::size_fn(),
                buffer_name: CAMERA_BUFFER_UNIFORM.to_string(),
                binding: 1,
                items_count: 1,
            },
        ];
        uniform_buffer_manager.create(BIND_GROUP_MAIN_BUFFERS, entries);

        // prepare pipeline
        let (_, main_buffers_uniform_bind_group_layout) = uniform_buffer_manager
            .borrow_bind_group(BIND_GROUP_MAIN_BUFFERS)
            .unwrap();

        let bind_group_layouts = vec![
            tex_manager.borrow_bind_group_layout(),
            main_buffers_uniform_bind_group_layout,
        ];
        let mut pipeline_manager = inner_managers.pipeline_manager.write().unwrap();
        pipeline_manager
            .create_pipeline(
                SPRITES_RENDER_PIPELINE_ID,
                MAIN_SHADERS_ID,
                &assets_manager,
                bind_group_layouts,
            )
            .unwrap();

        state_res.state = State::RENDER;
    }
}
