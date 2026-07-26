use glam::{Mat4, Vec3};
use specs::{Read, ReadStorage, System, Write};
use wgpu_core::ecs::{
    component::{player::Player, position::Position},
    resource::{
        buffers::BuffersResource,
        state::{State, StateResource},
    },
};

use crate::game::state::GameState;

pub(crate) struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        Read<'a, StateResource>,
        Write<'a, BuffersResource>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        let (state_res, mut buffers_res, player, position) = data;

        if state_res.state != State::RENDER {
            return;
        }

        let ortho_projection =
            glam::camera::lh::proj::directx::orthographic(0.0, 800.0, 600.0, 0.0, 0.1, 100.0);
        if state_res.game_state == GameState::MENU.to_string() {
            buffers_res.camera =
                ortho_projection * Mat4::from_translation(Vec3::new(0.0, 0.0, 0.1));
        }
        if state_res.game_state == GameState::LEVEL.to_string() {
            for (_, pos) in (&player, &position).join() {
                buffers_res.camera = ortho_projection
                    * Mat4::from_translation(Vec3::new(-(pos.x - 350.0), -(pos.y - 250.0), 0.1));
            }
        }
    }
}
