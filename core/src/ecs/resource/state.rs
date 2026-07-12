#[derive(Default, PartialEq, Debug)]
pub enum State {
    #[default]
    SCENE,
    RENDER,
}

#[derive(Default)]
pub struct StateResource {
    pub state: State,
    pub game_state: String,
}
