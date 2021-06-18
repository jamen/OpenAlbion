use std::array::IntoIter;

use winit::window::Window;

use crate::{SceneRenderer};

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

/// This helps initialize the renderer, so all these don't have to be passed around seperately
pub struct RendererBase {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swap_chain_descriptor: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
}

pub struct Renderer {
    base: RendererBase,
    scene_renderer: SceneRenderer,
}

/// The params passed to render systems.
pub struct RenderParams<'a, 'b, S> {
    pub base: &'a RendererBase,
    pub state: &'b State,
    pub frame: wgpu::SwapChainFrame,
}

/// Maybe this can be refactored with one big wgpu::Buffer and wgpu::BufferSlice's.
pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: Option<wgpu::Buffer>,
    count: u32,
}

/// Render systems own some graphics resources and do render passes.
pub trait RenderSystem<S> {
    fn render(&mut self, params: &RenderParams<S>) -> wgpu::CommandBuffer;
}

impl Renderer {
    pub async fn create(window: &Window) -> Self {
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

    pub fn render(&mut self) {
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
        };

        let command_bufs = [
            self.scene_renderer.render(&params),
            // self.gui_renderer.render(&params),
        ];

        self.base.queue.submit(IntoIter::new(command_bufs));
    }
}

impl RendererBase {
    /// Creates the surface, device, queue, and swapchain for a Window.
    pub async fn create(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();

        let (swap_chain_descriptor, swap_chain) =
            Self::create_swap_chain(&surface, &device, size.width, size.height);

        RendererBase {
            surface,
            device,
            queue,
            swap_chain_descriptor,
            swap_chain,
        }
    }

    pub(crate) fn create_swap_chain(
        surface: &wgpu::Surface,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> (wgpu::SwapChainDescriptor, wgpu::SwapChain) {
        // TODO: Query something better from winit or wgpu?
        let format = wgpu::TextureFormat::Bgra8Unorm;

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            format,
            height,
            width,
            present_mode: wgpu::PresentMode::Mailbox,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        };

        let swap_chain = device.create_swap_chain(surface, &swap_chain_descriptor);

        (swap_chain_descriptor, swap_chain)
    }
}