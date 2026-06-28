use specs::{System, Write};

use crate::{
    ecs::{resource::managers::ManagersResource, MAIN_SHADERS_ID, SPRITES_TEXTURE_ID},
    manager::asset_manager::AssetType,
};

pub struct Init;

impl<'a> System<'a> for Init {
    type SystemData = Write<'a, ManagersResource>;

    fn run(&mut self, data: Self::SystemData) {
        let inner_managers = data.get_managers().unwrap();
        let mut assets_manager = inner_managers.assets_manager.write().unwrap();

        // init assets
        assets_manager
            .add(
                SPRITES_TEXTURE_ID,
                AssetType::Texture {
                    path: "res/sprites.png".to_owned(),
                    width: 0,
                    height: 0,
                },
            )
            .unwrap();
        assets_manager
            .add(
                MAIN_SHADERS_ID,
                AssetType::Shader("res/main_shaders.wgsl".to_owned()),
            )
            .unwrap();
    }
}
