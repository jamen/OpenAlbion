use std::mem;
use std::num::NonZeroU64;

use crate::{Renderer,RendererBase,State,include_glsl};

pub struct ModelViewRenderer {
    g_buffer_model_1_pipeline: wgpu::RenderPipeline,
    light_pipeline: wgpu::RenderPipeline,
    scene_bind_group: wgpu::BindGroup,
    view_projection: BufferId,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    texture: wgpu::Buffer,
}

impl ModelViewRenderer {
    pub fn create(base: &RendererBase) -> Self {
        Self {
            g_buffer_model_1_pipeline: Self::create_g_buffer_model_1_pipeline(base),
            light_pipeline: Self::create_light_pipeline(base),
            model: None,
        }
    }

    pub fn create_g_buffer_model_1_pipeline(base: &RendererBase) -> wgpu::RenderPipeline {
        let scene_bind_group_layout = base.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("scene_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size:
                            Some(unsafe { NonZeroU64::new_unchecked(mem::size_of::<glam::Mat4>() as u64) }),
                    },
                    count: None,
                }
            ],
        });

        let scene_bind_group = base.device.create_bind_group(&wgpu::);

        let layout = base.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &base.material_bind_group_layout,
                &scene_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        base.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module:
                    &base.device.create_shader_module(&include_glsl!("src/shaders/g_buffer_model_1.vert", kind: vert)),
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module:
                    &base.device.create_shader_module(&include_glsl!("src/shaders/g_buffer_model_1.frag", kind: frag)),
                entry_point: "main",
                targets: &[],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        })
    }

    pub fn create_light_pipeline(base: &RendererBase) -> wgpu::RenderPipeline {
        let layout = base.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        base.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module:
                    &base.device.create_shader_module(&include_glsl!("src/shaders/light.vert", kind: vert)),
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module:
                    &base.device.create_shader_module(&include_glsl!("src/shaders/light.frag", kind: frag)),
                entry_point: "main",
                targets: &[],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        })
    }
}

impl Renderer {
    pub fn render_model_view(
        &mut self,
        frame: &wgpu::SwapChainFrame,
        encoder: &mut wgpu::CommandEncoder,
        state: &State,
    ) {
    }
}