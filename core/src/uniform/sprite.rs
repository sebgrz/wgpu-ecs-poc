use bytemuck::{Pod, Zeroable};

#[derive(Default, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Sprite {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub texture_clip: [f32; 4],
}
