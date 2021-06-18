mod base;
mod scene;

use std::array::IntoIter;

pub use base::*;
pub use scene::*;

use winit::window::Window;

use crate::State;

pub struct Renderer {
    base: RendererBase,
    scene_renderer: SceneRenderer,
}

impl Renderer {
    pub async fn create(window: &Window, _state: &State) -> Self {
        let base = RendererBase::create(window).await;
        let scene_renderer = SceneRenderer::create(&base);

        Self {
            base,
            scene_renderer,
        }
    }

    // TODO: Handle other events like scale factor change too.
    /// Resizes the swap chain.  This doesn't resize the render systems, which handle it on render instead. Maybe add to RenderSystem to handle these events
    pub fn resize(&mut self, width: u32, height: u32) {
        let (swap_chain_descriptor, swap_chain) =
            RendererBase::create_swap_chain(&self.base.surface, &self.base.device, width, height);
        self.base.swap_chain_descriptor = swap_chain_descriptor;
        self.base.swap_chain = swap_chain;
    }

    pub fn render(&mut self, state: &State) {
        let frame = match self.base.swap_chain.get_current_frame() {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Dropped frame. {}", e);
                return
            }
        };

        let params = RenderParams {
            base: &self.base,
            frame: frame,
            state: state,
        };

        let command_bufs = [
            self.scene_renderer.render(&params),
            // self.gui_renderer.render(&params),
        ];

        self.base.queue.submit(IntoIter::new(command_bufs));
    }
}