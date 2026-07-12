use specs::{Read, ReadStorage, System, Write};
use wgpu_core::{
    ecs::{
        component::position::Position,
        resource::{
            delta_time::DeltaTimeResource,
            input::InputResource,
            state::{State, StateResource},
        },
    },
    input::{KeyType, SpecialKey},
};

use crate::game::state::GameState;

pub(crate) struct MenuSystem;

impl<'a> System<'a> for MenuSystem {
    type SystemData = (
        Read<'a, DeltaTimeResource>,
        Read<'a, InputResource>,
        Write<'a, StateResource>,
        ReadStorage<'a, Position>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (_delta_time_res, input_res, mut state_res, _position) = data;
        if !(state_res.state == State::RENDER
            && state_res.game_state == GameState::MENU.to_string())
        {
            return;
        }

        if let Some(is_pressed) = input_res.keys.get(&KeyType::SPECIAL(SpecialKey::ENTER)) {
            if *is_pressed {
                println!("enter");
                state_res.state = State::SCENE;
                state_res.game_state = GameState::LEVEL.to_string();
            }
        }
    }
}
