use specs::{Read, ReadStorage, System};

use crate::{
    ecs::{
        component::{position::Position, tile::Tile},
        resource::managers::ManagersResource,
    },
    uniform::sprite::Sprite,
};

pub struct PreSpriteBuffer;

impl<'a> System<'a> for PreSpriteBuffer {
    type SystemData = (
        Read<'a, ManagersResource>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Tile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (managers_res, position, tile) = data;
        for (position, tile) in (&position, &tile).join() {
            println!("sprite: {:?} {:?}", position, tile);
            let _sprite = Sprite {
                x: position.x,
                y: position.y,
                width: 100,
                height: 100,
                texture_clip: tile.into_tex_dimensions((10, 10)),
            };
        }
    }
}
