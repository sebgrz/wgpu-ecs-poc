use specs::{Read, System, Write};

use crate::{
    ecs::{
        resource::{game::GameResource, managers::ManagersResource},
        MAIN_SHADERS_ID,
    },
    manager::asset_manager::AssetType,
};

pub struct Init;

impl<'a> System<'a> for Init {
    type SystemData = (Read<'a, GameResource>, Write<'a, ManagersResource>);

    fn run(&mut self, data: Self::SystemData) {
        let (game_res, managers_res) = data;
        let inner_managers = managers_res.get_managers().unwrap();
        let mut assets_manager = inner_managers.assets_manager.write().unwrap();

        // init assets

        for state in game_res.data.state_data.values() {
            assets_manager
                .add(
                    &state.texture_id,
                    AssetType::Texture {
                        path: state.texture_file_path.clone(),
                        width: 0,
                        height: 0,
                    },
                )
                .unwrap();
        }

        assets_manager
            .add(
                MAIN_SHADERS_ID,
                AssetType::Shader("res/main_shaders.wgsl".to_owned()),
            )
            .unwrap();
    }
}
