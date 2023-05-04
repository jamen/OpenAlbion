use std::mem;

use crate::{base::RenderPass, Base};

use bytemuck::{Pod, Zeroable};

pub const MODEL_VERT: &str = include_str!("../shaders/model.wgsl");

#[derive(Debug)]
pub enum ModelPipelineError {}

pub(crate) struct ModelPipeline {
    pub(crate) pipeline: wgpu::RenderPipeline,
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Vertex {
    pos: [f32; 4],
}

impl ModelPipeline {
    pub(crate) fn new(base: &Base) -> Result<ModelPipeline, ModelPipelineError> {
        let vert_shader = base
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(MODEL_VERT.into()),
            });

        let pipeline = base
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                vertex: wgpu::VertexState {
                    module: &vert_shader,
                    entry_point: "vs_main",
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<Vertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x4],
                    }],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &vert_shader,
                    entry_point: "fs_main",
                    targets: &[base.surface_config.format.into()],
                }),
                layout: None,
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
            });

        Ok(Self { pipeline })
    }
}