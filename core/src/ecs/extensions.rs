use specs::{world::EntitiesRes, LazyUpdate};

use crate::{
    data::entity_data::{ComponentData, EntityData},
    ecs::component::{
        animation::Animation, player::Player, position::Position, size::Size,
        sprite_animation::SpriteAnimation, tile::Tile,
    },
};

pub struct ECSExtensions;

impl ECSExtensions {
    pub fn spawn_entities(
        entities: &EntitiesRes,
        updater: &LazyUpdate,
        data_entities: &Vec<EntityData>,
    ) {
        for entity_data in data_entities {
            let entity = entities.create();
            for component in &entity_data.components {
                match component {
                    ComponentData::Player => {
                        updater.insert(entity, Player);
                    }
                    ComponentData::Size { width, height } => {
                        updater.insert(
                            entity,
                            Size {
                                width: *width,
                                height: *height,
                            },
                        );
                    }
                    ComponentData::Position { x, y } => {
                        updater.insert(entity, Position { x: *x, y: *y });
                    }
                    ComponentData::SpriteAnimation {
                        texture_id,
                        animation_id,
                        is_reversed,
                    } => {
                        updater.insert(
                            entity,
                            SpriteAnimation {
                                texture_id: texture_id.clone(),
                                animation_id: animation_id.clone(),
                                current_frame: 0,
                                current_duration: 0.0,
                                current_tile: None,
                                is_reversed: *is_reversed,
                            },
                        );
                    }
                    ComponentData::Tile {
                        texture_id,
                        x,
                        y,
                        x2,
                        y2,
                        is_reversed,
                    } => {
                        updater.insert(
                            entity,
                            Tile {
                                texture_id: texture_id.clone(),
                                x: *x,
                                y: *y,
                                x2: *x2,
                                y2: *y2,
                                is_reversed: *is_reversed,
                            },
                        );
                    }
                    ComponentData::Animation { animation_id } => updater.insert(
                        entity,
                        Animation {
                            animation_id: animation_id.clone(),
                            current_frame: 0,
                            current_duration: 0.0,
                        },
                    ),
                }
            }
        }
    }
}
