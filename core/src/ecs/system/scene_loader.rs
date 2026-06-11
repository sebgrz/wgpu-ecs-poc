use specs::{System, Write};

use crate::{
    ecs::{
        resource::managers::ManagersResource, MAIN_SHADERS_ID, SPRITES_BUFFER_UNIFORM,
        SPRITES_RENDER_PIPELINE_ID, SPRITES_TEXTURE_ID,
    },
    uniform::sprite::Sprite,
};

pub struct SceneLoader;

impl<'a> System<'a> for SceneLoader {
    type SystemData = Write<'a, ManagersResource>;

    fn run(&mut self, data: Self::SystemData) {
        let inner_managers = data.get_managers().unwrap();
        let assets_manager = inner_managers.assets_manager.read().unwrap();

        // load textures
        let mut tex_manager = inner_managers.texture_manager.write().unwrap();
        tex_manager
            .load_texture(&assets_manager, SPRITES_TEXTURE_ID)
            .unwrap();

        // prepare uniforms
        let mut uniform_buffer_manager = inner_managers.uniform_buffer_manager.write().unwrap();
        uniform_buffer_manager.create::<Sprite>(SPRITES_BUFFER_UNIFORM, 1024);

        // prepare pipeline
        let (_, sprites_buffer_uniform_bind_group_layout) = uniform_buffer_manager
            .borrow_bind_group(SPRITES_BUFFER_UNIFORM)
            .unwrap();
        let bind_group_layouts = vec![
            tex_manager.borrow_bind_group_layout(),
            sprites_buffer_uniform_bind_group_layout,
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
    }
}
