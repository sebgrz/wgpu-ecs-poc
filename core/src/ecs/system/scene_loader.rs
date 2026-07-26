use glam::Mat4;
use specs::{Read, System, Write};

use crate::{
    ecs::{
        resource::{
            game::GameResource,
            managers::ManagersResource,
            state::{State, StateResource},
        },
        CAMERA_BUFFER_UNIFORM, MAIN_SHADERS_ID, SPRITES_BUFFER_UNIFORM, SPRITES_RENDER_PIPELINE_ID,
    },
    uniform::sprite::Sprite,
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

        uniform_buffer_manager.create::<Sprite>(SPRITES_BUFFER_UNIFORM, 1024);
        uniform_buffer_manager.create::<Mat4>(CAMERA_BUFFER_UNIFORM, 1);

        // prepare pipeline
        let (_, sprites_buffer_uniform_bind_group_layout) = uniform_buffer_manager
            .borrow_bind_group(SPRITES_BUFFER_UNIFORM)
            .unwrap();
        let (_, camera_buffer_uniform_bind_group_layout) = uniform_buffer_manager
            .borrow_bind_group(CAMERA_BUFFER_UNIFORM)
            .unwrap();

        let bind_group_layouts = vec![
            tex_manager.borrow_bind_group_layout(),
            sprites_buffer_uniform_bind_group_layout,
            camera_buffer_uniform_bind_group_layout,
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
