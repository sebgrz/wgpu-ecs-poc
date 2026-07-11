use glam::Vec3;
use specs::{
    storage::GenericWriteStorage, Entities, Read, ReadStorage, System, Write, WriteStorage,
};
use wgpu_core::ecs::{
    component::{player::Player, position::Position},
    resource::{
        buffers::BuffersResource,
        input::InputResource,
        state::{State, StateResource},
    },
};

pub(crate) struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, InputResource>,
        Read<'a, StateResource>,
        Write<'a, BuffersResource>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (entities, input_res, state_res, mut buffers_res, player, mut position) = data;

        if state_res.state != State::RENDER {
            return;
        }

        if let Some((entity, _)) = (&entities, &player).join().next() {
            let player_pos = position.get_mut(entity).unwrap();
            for (key, is_pressed) in &input_res.keys {
                if *is_pressed {
                    if let wgpu_core::input::KeyType::SPECIAL(special_key) = key {
                        match special_key {
                            wgpu_core::input::SpecialKey::LEFT => player_pos.x -= 1,
                            wgpu_core::input::SpecialKey::RIGHT => player_pos.x += 1,
                            wgpu_core::input::SpecialKey::UP => player_pos.y -= 1,
                            wgpu_core::input::SpecialKey::DOWN => player_pos.y += 1,
                            _ => {}
                        }
                    }
                }
            }
            buffers_res.camera = Vec3::new(
                -(player_pos.x - 350) as f32,
                -(player_pos.y - 250) as f32,
                0.1,
            )
        }
    }
}
