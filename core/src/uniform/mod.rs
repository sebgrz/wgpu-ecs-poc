use glam::Mat4;

use crate::uniform::sprite::Sprite;

pub mod sprite;

pub type SizedFn = fn() -> u64;

pub trait UniformBufSize: Sized {
    fn size_fn() -> SizedFn {
        || std::mem::size_of::<Self>() as u64
    }
}

impl UniformBufSize for Sprite {}
impl UniformBufSize for Mat4 {}
