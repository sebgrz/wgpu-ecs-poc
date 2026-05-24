use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
