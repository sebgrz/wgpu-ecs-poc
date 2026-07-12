use std::sync::{Arc, RwLock};

use specs::{World, WorldExt};

use crate::{
    ecs::{
        component::{player::Player, position::Position, size::Size, tile::Tile},
        resource::{
            buffers::BuffersResource, delta_time::DeltaTimeResource, input::InputResource,
            managers::ManagersResource, renderer::RendererResource, state::StateResource,
        },
    },
    manager::{
        asset_manager::AssetManager, pipeline_manager::PipelineManager,
        texture_manager::TextureManager, uniform_buffer_manager::UniformBufferManager,
    },
    renderer::{Renderer, SharedRenderer},
};

pub mod ecs;
pub mod input;
pub mod manager;
pub mod renderer;
pub mod uniform;
pub mod window;

pub type SharedWorld = Arc<RwLock<World>>;

pub fn init() -> (SharedRenderer, SharedWorld) {
    let renderer = Arc::new(RwLock::new(Renderer::default()));
    let mut world = World::new();
    world.register::<Size>();
    world.register::<Position>();
    world.register::<Tile>();
    world.register::<Player>();

    (renderer.clone(), Arc::new(RwLock::new(world)))
}

pub fn init_managers(world: &mut World, renderer: SharedRenderer) {
    let asset_manager = Arc::new(RwLock::new(AssetManager::new()));
    let buffer_manager = Arc::new(RwLock::new(UniformBufferManager::new(renderer.clone())));
    let texture_manager = Arc::new(RwLock::new(TextureManager::new(renderer.clone())));
    let pipeline_manager = Arc::new(RwLock::new(PipelineManager::new(renderer.clone())));

    world.insert(ManagersResource::new(
        asset_manager,
        texture_manager,
        buffer_manager,
        pipeline_manager,
    ));
    world.insert(RendererResource {
        renderer: Some(renderer.clone()),
    });
    world.insert(BuffersResource::default());
    world.insert(StateResource::default());
    world.insert(InputResource::default());
    world.insert(DeltaTimeResource::default());
}
