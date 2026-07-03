#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpecialKey {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    ENTER,
    SPACE,
    ESCAPE,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyType {
    SPECIAL(SpecialKey),
    CHAR(String),
}

#[derive(Debug)]
pub struct KeyboardInputAction {
    pub key: KeyType,
    pub is_pressed: bool,
}
