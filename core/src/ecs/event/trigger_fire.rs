use specs::Entity;

pub struct TriggerFireEvent {
    pub trigger_name: String,
    pub entity: Entity,
}
