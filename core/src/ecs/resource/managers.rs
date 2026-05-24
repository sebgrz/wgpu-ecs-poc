use std::rc::Rc;

use crate::manager::texture_manager::TextureManager;

pub struct Managers {
    texture_manager: Rc<TextureManager>,
}
