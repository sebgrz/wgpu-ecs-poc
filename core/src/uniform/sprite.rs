use bytemuck::{Pod, Zeroable};

#[derive(Default, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub texture_clip: [f32; 4],
}
