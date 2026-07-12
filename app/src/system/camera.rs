use glam::Vec3;
use specs::{Read, ReadStorage, System, Write};
use wgpu_core::ecs::{
    component::{player::Player, position::Position},
    resource::{
        buffers::BuffersResource,
        state::{State, StateResource},
    },
};

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

        for (_, pos) in (&player, &position).join() {
            buffers_res.camera = Vec3::new(-(pos.x - 350.0), -(pos.y - 250.0), 0.1);
        }
    }
}
