mod gui;
mod level_view;
mod model_view;
mod model_manager;

pub use gui::*;
pub use level_view::*;
pub use model_view::*;
pub use model_manager::*;

use winit::window::Window;

use crate::{State,Page};

pub struct RendererBase {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub struct Renderer {
    base: RendererBase,
    swap_chain: wgpu::SwapChain,
    swap_chain_descriptor: wgpu::SwapChainDescriptor,
    material_bind_group_layout: wgpu::BindGroupLayout,
    model_manager: ModelManager,
    gui_renderer: GuiRenderer,
    level_view_renderer: LevelViewRenderer,
    model_view_renderer: ModelViewRenderer,
}

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

impl RendererBase {
    async fn create(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            }, None)
            .await
            .unwrap();

        let material_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("material_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: false,
                        comparison: false,
                    },
                    count: None,
                },
            ],
        });

        let resources = Resources::create();

        Self {
            surface,
            device,
            queue,
            material_bind_group_layout,
            resources,
        }
    }
}

impl Renderer {
    pub async fn create(window: &Window) -> Self {
        let base = RendererBase::create(window).await;

        let (swap_chain_descriptor, swap_chain) = Self::new_swap_chain(&base, window);

        let gui_renderer = GuiRenderer::create(&base);
        let level_view_renderer = LevelViewRenderer::create(&base);
        let model_view_renderer = ModelViewRenderer::create(&base);

        Self {
            base,
            swap_chain,
            swap_chain_descriptor,
            gui_renderer,
            level_view_renderer,
            model_view_renderer,
        }
    }

    fn new_swap_chain(base: &RendererBase, window: &Window) -> (wgpu::SwapChainDescriptor, wgpu::SwapChain) {
        let size = window.inner_size();

        // TODO: Query something better from winit or wgpu?
        let format = wgpu::TextureFormat::Bgra8Unorm;

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            format,
            height: size.height,
            width: size.width,
            present_mode: wgpu::PresentMode::Mailbox,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        };

        let swap_chain = base.device.create_swap_chain(&base.surface, &swap_chain_descriptor);

        (swap_chain_descriptor, swap_chain)
    }

    pub fn update_swap_chain(&mut self, window: &Window) {
        let (swap_chain_descriptor, swap_chain) = Self::new_swap_chain(&self.base, &window);
        self.swap_chain_descriptor = swap_chain_descriptor;
        self.swap_chain = swap_chain;
    }

    pub fn render(&mut self, state: &State) {
        let frame = match self.swap_chain.get_current_frame() {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Dropped frame. {}", e);
                return
            }
        };

        let mut encoder = self.base.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        match state.page {
            Page::ModelView => self.render_model_view(&frame, &mut encoder, &state),
            Page::LevelView => self.render_level_view(&frame, &mut encoder, &state),
            _ => {}
        }

        self.render_gui(&frame, &mut encoder, &state);

        self.base.queue.submit(Some(encoder.finish()));
    }
}