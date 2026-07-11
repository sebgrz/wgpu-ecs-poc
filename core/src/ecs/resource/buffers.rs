use std::array;

use glam::Vec3;

use crate::uniform::sprite::Sprite;

pub struct BuffersResource {
    pub camera: Vec3,
    pub sprites: [Sprite; 1024],
    pub sprites_size: usize,
}

impl Default for BuffersResource {
    fn default() -> Self {
        Self {
            camera: Vec3::ZERO,
            sprites: array::from_fn(|_| Sprite::default()),
            sprites_size: 0,
        }
    }
}
