use std::array;

use crate::uniform::sprite::Sprite;

pub struct SpritesBufferResource {
    pub sprites: [Sprite; 1024],
    pub size: u32,
}

impl Default for SpritesBufferResource {
    fn default() -> Self {
        Self {
            sprites: array::from_fn(|_| Sprite::default()),
            size: 0,
        }
    }
}
