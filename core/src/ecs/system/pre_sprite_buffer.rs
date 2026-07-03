use specs::{Read, ReadStorage, System, Write};

use crate::{
    ecs::{
        component::{position::Position, tile::Tile},
        resource::{
            managers::ManagersResource,
            sprites_buffer::SpritesBufferResource,
            state::{State, StateResource},
        },
    },
    uniform::sprite::Sprite,
};

pub struct PreSpriteBuffer;

impl<'a> System<'a> for PreSpriteBuffer {
    type SystemData = (
        Read<'a, ManagersResource>,
        Read<'a, StateResource>,
        Write<'a, SpritesBufferResource>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Tile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (managers_res, state_res, mut sprites_buffer_res, position, tile) = data;

        if state_res.state != State::RENDER {
            return;
        }

        let inner_managers = managers_res.get_managers().unwrap();
        let tex_manager = inner_managers.texture_manager.read().unwrap();

        let mut count = 0;
        for (position, tile) in (&position, &tile).join() {
            println!("sprite: {:?} {:?}", position, tile);
            if let Some(size) = tex_manager.borrow_size_cache(&tile.texture_id) {
                let sprite = Sprite {
                    x: position.x,
                    y: position.y,
                    width: 100,  // TODO
                    height: 100, // TODO
                    texture_clip: tile.into_tex_dimensions(size.clone()),
                };
                sprites_buffer_res.sprites[count] = sprite;
                count += 1;
            }
        }

        sprites_buffer_res.size = count;
    }
}
