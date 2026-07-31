use specs::{shrev::EventChannel, Read, System};

use crate::ecs::{event::trigger_fire::TriggerFireEvent, resource::state::StateResource};

pub(crate) struct TriggersLauncherSystem;

impl<'a> System<'a> for TriggersLauncherSystem {
    type SystemData = (
        Read<'a, StateResource>,
        Read<'a, EventChannel<TriggerFireEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        todo!()
    }
}
