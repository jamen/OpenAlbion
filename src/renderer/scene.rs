use std::{convert::TryInto, mem};

use crate::{RenderParams, RenderSystem, RendererBase, State};

use glam::Mat4;

pub struct SceneRenderer {
    mvp_buffer: wgpu::Buffer,
    // model: Model,
}

pub struct Model {
    material: Material,
    mesh: Mesh,
}

pub struct Material {
    base_color: wgpu::Texture,
}

pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    count: u32,
}

impl SceneRenderer {
    pub fn create(base: &RendererBase) -> Self {
        let mvp_buffer = base.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: mem::size_of::<glam::Mat4>().try_into().unwrap(),
            usage: wgpu::BufferUsage::UNIFORM,
            mapped_at_creation: false,
        });

        Self { mvp_buffer }
    }
}

impl SceneRenderer {
    pub fn render(&mut self, params: &RenderParams<'_, '_, State>) -> wgpu::CommandBuffer {
        let RenderParams { base, frame, .. } = params;

        let mut encoder = base
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // {
        //     let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        //         label: Some("test"),
        //         color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
        //             attachment: &frame.output.view,
        //             resolve_target: None,
        //             ops: wgpu::Operations {
        //                 load: wgpu::LoadOp::Clear(wgpu::Color {
        //                     r: 0.1,
        //                     g: 0.2,
        //                     b: 0.3,
        //                     a: 1.0,
        //                 }),
        //                 store: true,
        //             },
        //         }],
        //         depth_stencil_attachment: None,
        //     });

        //     rpass.set_pipeline(&self.g_buffer_pipeline);
        //     rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        //     rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        //     rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
        // }

        encoder.finish()
    }
}
