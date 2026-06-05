use crate::renderer::SharedRenderer;

#[derive(Default)]
pub struct RendererResource {
    pub renderer: Option<SharedRenderer>,
}
