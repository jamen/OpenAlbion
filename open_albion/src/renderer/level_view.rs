use crate::{Renderer,RendererBase,State};

pub struct LevelViewRenderer {

}

impl LevelViewRenderer {
    pub fn create(base: &RendererBase) -> Self {
        Self {

        }
    }
}

impl Renderer {
    pub fn render_level_view(
        &mut self,
        frame: &wgpu::SwapChainFrame,
        encoder: &mut wgpu::CommandEncoder,
        state: &State,
    ) {
    }
}