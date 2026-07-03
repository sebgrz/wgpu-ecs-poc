#[derive(Default, PartialEq)]
pub enum State {
    #[default]
    SCENE,
    RENDER,
}

#[derive(Default)]
pub struct StateResource {
    pub state: State,
}
