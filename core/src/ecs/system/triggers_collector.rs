use specs::{
    shrev::EventChannel, Entities, Entity, Join, Read, ReadStorage, System, Write, WriteStorage,
};

use crate::{
    data::trigger::{Trigger, TriggerAction},
    ecs::{
        component::{marker::Marker, position::Position},
        event::trigger_fire::TriggerFireEvent,
        resource::{
            delta_time::DeltaTimeResource,
            input::InputResource,
            state::{State, StateResource},
        },
    },
    input::KeyType,
};

struct TriggerCtx<'a> {
    entity: &'a Entity,
    position: Option<&'a Position>,
    marker: Option<&'a Marker>,
    keys_pressed: Vec<&'a KeyType>,
}

pub(crate) struct TriggersCollectorSystem;

impl<'a> System<'a> for TriggersCollectorSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, InputResource>,
        Read<'a, StateResource>,
        Write<'a, EventChannel<TriggerFireEvent>>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Marker>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, input_res, state_res, mut channel, position, marker) = data;

        if state_res.state != State::RENDER && state_res.level_data.is_none() {
            return;
        }

        let level = state_res.level_data.as_ref().unwrap();

        let entities: Vec<Entity> = entities.join().collect();
        for entity in entities {
            let ctx = TriggerCtx {
                entity: &entity,
                position: position.get(entity),
                marker: marker.get(entity),
                keys_pressed: input_res
                    .keys
                    .iter()
                    .filter(|(_, v)| **v == true)
                    .map(|(k, _)| k)
                    .collect(),
            };
            for (name, trigger) in &level.triggers {
                let to_take = trigger.actions.iter().all(|a| a.eval(&ctx));
                if to_take {
                    let event = TriggerFireEvent {
                        trigger_name: name.clone(),
                        entity: entity,
                    };
                    channel.single_write(event);
                }
            }
        }
    }
}

impl TriggerAction {
    fn eval<'a>(&self, context: &'a TriggerCtx<'a>) -> bool {
        match self {
            TriggerAction::Box { x, y, x1, y1 } => {
                let p = match context.position {
                    Some(p) => p,
                    None => return false,
                }; // fail fast if no Position
                p.x >= *x && p.x <= *x1 && p.y >= *y && p.y <= *y1
            }
            TriggerAction::KeyDown { key_type } => context.keys_pressed.contains(&key_type),
            TriggerAction::Marker { name } => {
                if let Some(marker) = context.marker {
                    marker.name.contains(name)
                } else {
                    return false;
                }
            }
        }
    }
}
