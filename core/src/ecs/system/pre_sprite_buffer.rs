use specs::{ReadStorage, System};

use crate::ecs::component::sprite::Sprite;

pub struct PreSpriteBuffer;

impl<'a> System<'a> for PreSpriteBuffer {
    type SystemData = ReadStorage<'a, Sprite>;

    fn run(&mut self, sprites: Self::SystemData) {
        use specs::Join;
        for sprite in sprites.join() {
            println!("sprite: {:?}", sprite);
        }
    }
}
