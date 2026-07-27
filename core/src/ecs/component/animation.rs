use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Animation {
    pub texture_id: String,
    pub animation_id: String,
    pub current_frame: usize,
    pub current_duration: f32,
    pub current_tile: Option<[u32; 4]>,
    pub is_reversed: bool,
}

impl Animation {
    pub fn into_tex_dimensions(&self, tex_size: (u32, u32)) -> [f32; 4] {
        let tile = self.current_tile.unwrap();
        if self.is_reversed {
            [
                (tile[2] as f32) / (tex_size.0 as f32),
                (tile[1] as f32) / (tex_size.1 as f32),
                (tile[0] as f32) / (tex_size.0 as f32),
                (tile[3] as f32) / (tex_size.1 as f32),
            ]
        } else {
            [
                (tile[0] as f32) / (tex_size.0 as f32),
                (tile[1] as f32) / (tex_size.1 as f32),
                (tile[2] as f32) / (tex_size.0 as f32),
                (tile[3] as f32) / (tex_size.1 as f32),
            ]
        }
    }
}
