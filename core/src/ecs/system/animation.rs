use specs::Join;
use specs::{Read, System, WriteStorage};

use crate::ecs::resource::delta_time::DeltaTimeResource;
use crate::ecs::{component::sprite_animation::SpriteAnimation, resource::game::GameResource};

pub struct AnimationSystem;

impl<'a> System<'a> for AnimationSystem {
    type SystemData = (
        Read<'a, GameResource>,
        Read<'a, DeltaTimeResource>,
        WriteStorage<'a, SpriteAnimation>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (game_res, delta_time_res, mut animation) = data;

        for a in (&mut animation).join() {
            let animation_data = &game_res.data.sprite_animations[&a.animation_id];
            let count = &animation_data.keyframes.len();
            let mut keyframe = &animation_data.keyframes[a.current_frame];

            if a.current_duration >= keyframe.duration {
                let next_frame = a.current_frame + 1;
                if next_frame < *count {
                    a.current_frame = next_frame;
                    a.current_duration = 0.0;
                } else if animation_data.looping {
                    a.current_frame = 0;
                    a.current_duration = 0.0;
                } else {
                    a.current_duration = 0.0;
                }
                keyframe = &animation_data.keyframes[a.current_frame];
            }

            a.current_tile = Some(keyframe.tile);
            a.current_duration += delta_time_res.time.as_secs_f32();
        }
    }
}
