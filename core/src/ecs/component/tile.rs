use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Tile {
    pub texture_id: String,
    pub x: i32,
    pub y: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Tile {
    pub fn into_tex_dimensions(&self, tex_size: (u32, u32)) -> [f32; 4] {
        [
            (self.x as f32) / (tex_size.0 as f32),
            (self.y as f32) / (tex_size.1 as f32),
            (self.x2 as f32) / (tex_size.0 as f32),
            (self.y2 as f32) / (tex_size.1 as f32),
        ]
    }
}
