pub mod component;
pub mod resource;
pub mod system;

// buffers
pub(crate) static SPRITES_BUFFER_UNIFORM: &str = "sprites_buffer";
pub(crate) static CAMERA_BUFFER_UNIFORM: &str = "camera_buffer";

// textures
pub static SPRITES_TEXTURE_ID: &str = "sprites_texture";
pub static MENU_TEXTURE_ID: &str = "menu_texture";

// other
pub(crate) static SPRITES_RENDER_PIPELINE_ID: &str = "sprite_render_pipeline";
pub(crate) static MAIN_SHADERS_ID: &str = "main_shaders";
