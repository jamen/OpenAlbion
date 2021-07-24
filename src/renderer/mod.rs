mod base;
mod scene;

pub use base::*;
pub use scene::*;

use crate::State;

use std::array::IntoIter;

use winit::window::Window;

/// Compiles and embeds a shader.
#[macro_export]
macro_rules! include_glsl {
    ($path:literal, $($token:tt)*) => {
        wgpu::ShaderModuleDescriptor {
            label: Some($path),
            source: wgpu::ShaderSource::SpirV(vk_shader_macros::include_glsl!($path, $($token)*)[..].into()),
            flags: wgpu::ShaderFlags::VALIDATION,
        }
    }
}

pub struct Renderer {
    pub base: RendererBase,
    pub scene_renderer: SceneRenderer,
}

/// Maybe this can be refactored with one big wgpu::Buffer and wgpu::BufferSlice's.
pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: Option<wgpu::Buffer>,
    count: u32,
}

impl Renderer {
    pub async fn create(window: &Window, state: &State) -> Self {
        let base = RendererBase::create(window).await;
        let scene_renderer = SceneRenderer::create(&base, &state);

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

        let command_bufs = [
            self.scene_renderer.render(&self.base, &frame, &state),
            // self.gui_renderer.render(&params),
        ];

        self.base.queue.submit(IntoIter::new(command_bufs));
    }
}

