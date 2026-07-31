use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Marker {
    pub name: String, // TODO: multiple names
}
