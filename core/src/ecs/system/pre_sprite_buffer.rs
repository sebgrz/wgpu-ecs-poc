use specs::{Read, ReadStorage, System, Write};

use crate::{
    ecs::{
        component::{position::Position, size::Size, tile::Tile},
        resource::{
            buffers::BuffersResource,
            managers::ManagersResource,
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
        Write<'a, BuffersResource>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Tile>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (managers_res, state_res, mut buffers_res, size, position, tile) = data;

        if state_res.state != State::RENDER {
            return;
        }

        let inner_managers = managers_res.get_managers().unwrap();
        let tex_manager = inner_managers.texture_manager.read().unwrap();

        let mut count = 0;
        for (size, position, tile) in (&size, &position, &tile).join() {
            if let Some(tex_size) = tex_manager.borrow_size_cache(&tile.texture_id) {
                let sprite = Sprite {
                    x: position.x,
                    y: position.y,
                    width: size.width,
                    height: size.height,
                    texture_clip: tile.into_tex_dimensions(tex_size.clone()),
                };
                buffers_res.sprites[count] = sprite;
                count += 1;
            }
        }

        buffers_res.sprites_size = count;
    }
}
