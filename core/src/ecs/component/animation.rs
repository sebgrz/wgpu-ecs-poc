use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Animation {
    pub animation_id: String,
    pub current_frame: usize,
    pub current_duration: f32,
}
