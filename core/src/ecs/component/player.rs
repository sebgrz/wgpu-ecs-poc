use specs::{Component, NullStorage};

#[derive(Component, Debug)]
#[storage(NullStorage)]
pub struct Player;
