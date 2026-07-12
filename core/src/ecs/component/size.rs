use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
