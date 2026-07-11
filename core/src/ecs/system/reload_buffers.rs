use specs::{Read, System};

use crate::ecs::{
    resource::{
        buffers::BuffersResource,
        managers::ManagersResource,
        state::{State, StateResource},
    },
    CAMERA_BUFFER_UNIFORM, SPRITES_BUFFER_UNIFORM,
};

pub struct ReloadBuffers;

impl<'a> System<'a> for ReloadBuffers {
    type SystemData = (
        Read<'a, ManagersResource>,
        Read<'a, BuffersResource>,
        Read<'a, StateResource>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (managers_resource, buffers_resources, state_res) = data;

        if state_res.state != State::RENDER {
            return;
        }
        let inner_mangers = managers_resource.get_managers().unwrap();

        let uniform_buffer_manager = inner_mangers.uniform_buffer_manager.read().unwrap();
        // write sprites to gpu buffer
        let sprites = buffers_resources.sprites;
        let fragment_sprites = &sprites[0..buffers_resources.sprites_size];

        uniform_buffer_manager
            .write_from_beginning(SPRITES_BUFFER_UNIFORM, fragment_sprites.to_vec());
        uniform_buffer_manager.insert(CAMERA_BUFFER_UNIFORM, &buffers_resources.camera, 0);
    }
}
