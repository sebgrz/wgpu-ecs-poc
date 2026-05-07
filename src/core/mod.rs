use specs::{World, WorldExt as _};

pub(crate) mod manager;
pub(crate) mod renderer;
pub(crate) mod window;

pub(crate) fn init() {
    let _renderer_world = World::new();
}
