use std::fmt::Display;

#[derive(Debug)]
pub(crate) enum GameState {
    MENU,
    LEVEL,
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
