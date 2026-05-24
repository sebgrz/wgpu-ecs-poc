use specs::{ReadStorage, System};

use crate::{
    ecs::component::{position::Position, tile::Tile},
    uniform::sprite::Sprite,
};

pub struct PreSpriteBuffer;

impl<'a> System<'a> for PreSpriteBuffer {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Tile>);

    fn run(&mut self, (position, tile): Self::SystemData) {
        use specs::Join;
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
