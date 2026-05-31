use specs::{Read, System};

use crate::ecs::resource::{managers::ManagersResource, sprites_buffer::SpritesBufferResource};

pub struct SpriteRenderer;

impl<'a> System<'a> for SpriteRenderer {
    type SystemData = (Read<'a, SpritesBufferResource>, Read<'a, ManagersResource>);

    fn run(&mut self, _data: Self::SystemData) {
        todo!()
    }
}
