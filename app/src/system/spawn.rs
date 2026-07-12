use specs::Join;
use specs::{Entities, Entity, Read, System, WriteStorage};
use wgpu_core::ecs::{
    component::{player::Player, position::Position, size::Size, tile::Tile},
    resource::state::{State, StateResource},
};
use wgpu_core::ecs::{MENU_TEXTURE_ID, SPRITES_TEXTURE_ID};

use crate::game::state::GameState;

pub(crate) struct SpawnSystem;

impl<'a> System<'a> for SpawnSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, StateResource>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, Tile>,
        WriteStorage<'a, Player>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, state_res, mut position, mut size, mut tile, mut player) = data;
        if state_res.state != State::SCENE {
            return;
        }
        let to_delete: Vec<Entity> = entities.join().collect();
        for entity in to_delete {
            entities.delete(entity).expect("delete entities");
        }

        // create entities
        if state_res.game_state == GameState::MENU.to_string() {
            entities
                .build_entity()
                .with(
                    Size {
                        width: 290.0,
                        height: 90.0,
                    },
                    &mut size,
                )
                .with(Position { x: 255.0, y: 255.0 }, &mut position)
                .with(
                    Tile {
                        texture_id: MENU_TEXTURE_ID.to_owned(),
                        x: 0,
                        y: 0,
                        x2: 580,
                        y2: 180,
                    },
                    &mut tile,
                )
                .build();
        }
        if state_res.game_state == GameState::LEVEL.to_string() {
            entities
                .build_entity()
                .with(Player, &mut player)
                .with(
                    Size {
                        width: 100.0,
                        height: 150.0,
                    },
                    &mut size,
                )
                .with(Position { x: 10.0, y: 20.0 }, &mut position)
                .with(
                    Tile {
                        texture_id: SPRITES_TEXTURE_ID.to_owned(),
                        x: 3,
                        y: 16,
                        x2: 15,
                        y2: 32,
                    },
                    &mut tile,
                )
                .build();

            entities
                .build_entity()
                .with(
                    Size {
                        width: 100.0,
                        height: 100.0,
                    },
                    &mut size,
                )
                .with(Position { x: 200.0, y: 125.0 }, &mut position)
                .with(
                    Tile {
                        texture_id: "sprites_texture".to_owned(),
                        x: 19,
                        y: 5,
                        x2: 27,
                        y2: 16,
                    },
                    &mut tile,
                )
                .build();
        }
    }
}
