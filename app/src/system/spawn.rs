use specs::Join;
use specs::{Entities, Entity, LazyUpdate, Read, System};
use wgpu_core::ecs::extensions::ECSExtensions;
use wgpu_core::ecs::resource::game::GameResource;
use wgpu_core::ecs::resource::managers::ManagersResource;
use wgpu_core::ecs::resource::state::{State, StateResource};

use crate::game::state::GameState;

pub(crate) struct SpawnSystem;

impl<'a> System<'a> for SpawnSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, StateResource>,
        Read<'a, ManagersResource>,
        Read<'a, GameResource>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, lazy_update_res, state_res, managers_res, game_res) = data;
        if state_res.state != State::SCENE {
            return;
        }
        let to_delete: Vec<Entity> = entities.join().collect();
        for entity in to_delete {
            entities.delete(entity).expect("delete entities");
        }
        let managers = managers_res.get_managers().unwrap();
        let file_manager = managers.file_manager.read().unwrap();

        // create entities
        if state_res.game_state == GameState::MENU.to_string() {
            ECSExtensions::spawn_entities(
                &entities,
                &lazy_update_res,
                &game_res.data.state_data[&state_res.game_state].entities,
            );
        }
        if state_res.game_state == GameState::LEVEL.to_string() {
            let level = file_manager.load_level(1);
            ECSExtensions::spawn_entities(&entities, &lazy_update_res, &level.entities);
        }
    }
}
