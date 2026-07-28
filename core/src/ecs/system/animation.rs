use glam::Vec2;
use specs::Join;
use specs::{Read, System, WriteStorage};

use crate::data::game_data::KeyframeData;
use crate::ecs::component::animation::Animation;
use crate::ecs::component::position::Position;
use crate::ecs::resource::delta_time::DeltaTimeResource;
use crate::ecs::{component::sprite_animation::SpriteAnimation, resource::game::GameResource};

pub struct AnimationSystem;

impl<'a> System<'a> for AnimationSystem {
    type SystemData = (
        Read<'a, GameResource>,
        Read<'a, DeltaTimeResource>,
        WriteStorage<'a, SpriteAnimation>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Animation>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (game_res, delta_time_res, mut sprite_animation, mut position, mut animation) = data;

        for a in (&mut sprite_animation).join() {
            let animation_data = &game_res.data.sprite_animations[&a.animation_id];
            let count = &animation_data.keyframes.len();
            let mut keyframe = &animation_data.keyframes[a.current_frame];

            let mut should_update_current_duration = true;
            if a.current_duration >= keyframe.duration {
                let next_frame = a.current_frame + 1;
                if next_frame < *count {
                    a.current_frame = next_frame;
                    a.current_duration = 0.0;
                } else if animation_data.looping {
                    a.current_frame = 0;
                    a.current_duration = 0.0;
                } else {
                    should_update_current_duration = false;
                }
                keyframe = &animation_data.keyframes[a.current_frame];
            }

            if !should_update_current_duration {
                return;
            }
            a.current_tile = Some(keyframe.tile);
            a.current_duration += delta_time_res.time.as_secs_f32();
        }

        for (p, a) in (&mut position, &mut animation).join() {
            let animation_data = &game_res.data.animations[&a.animation_id];
            let count = &animation_data.keyframes.len();
            let mut keyframe = &animation_data.keyframes[a.current_frame];

            let mut should_update_current_duration = true;
            if a.current_duration >= keyframe.duration {
                let next_frame = a.current_frame + 1;
                if next_frame < *count {
                    a.current_frame = next_frame;
                    a.current_duration = 0.0;
                } else if animation_data.looping {
                    a.current_frame = 0;
                    a.current_duration = 0.0;
                } else {
                    should_update_current_duration = false;
                }
                keyframe = &animation_data.keyframes[a.current_frame];
            }

            if !should_update_current_duration {
                return;
            }

            let current_position = count_current_position(&keyframe, &a.current_duration);
            p.x = current_position[0];
            p.y = current_position[1];
            a.current_duration += delta_time_res.time.as_secs_f32();
        }

        fn count_current_position(keyframe: &KeyframeData, current_duration: &f32) -> [f32; 2] {
            let time = (current_duration / keyframe.duration).clamp(0.0, 1.0);
            let start = Vec2::new(keyframe.start_pos[0], keyframe.start_pos[1]);
            let end = Vec2::new(keyframe.end_pos[0], keyframe.end_pos[1]);

            let current_position = start.lerp(end, time);
            [current_position.x, current_position.y]
        }
    }
}
