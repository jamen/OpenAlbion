use std::{convert::TryInto, mem};

use crate::{Mesh, RenderParams, RendererBase, Scene};

use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub struct SceneRenderer {
    model_view_projection: wgpu::Buffer,
    /// TODO: Replace with many models. Rendering just one for now.
    model: Model,
}

pub struct Model {
    primitives: Vec<Primitive>,
}

pub struct Primitive {
    mesh: Mesh,
    // material: Material,
}

pub struct Material {
    base_color: wgpu::Texture,
}

impl SceneRenderer {
    pub fn create(base: &RendererBase, scene: &Scene) -> Self {
        let model_view_projection = base.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: mem::size_of::<glam::Mat4>().try_into().unwrap(),
            usage: wgpu::BufferUsage::UNIFORM,
            mapped_at_creation: false,
        });

        let model = Self::load_model(base, &scene.model);

        Self {
            model_view_projection,
            model,
        }
    }

    fn load_model(base: &RendererBase, model: &fable_data::Model) -> Model {
        let primitives = model
            .primitives
            .iter()
            .map(|primitive| {
                let vertex_buffer = base.device.create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&primitive.vertices),
                    usage: wgpu::BufferUsage::COPY_SRC | wgpu::BufferUsage::VERTEX,
                });

                let index_buffer = Some(base.device.create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: &primitive.index_buffer,
                    usage: wgpu::BufferUsage::COPY_SRC | wgpu::BufferUsage::INDEX,
                }));

                let mesh = Mesh {
                    vertex_buffer,
                    index_buffer,
                    count: primitive.index_count,
                };

                // let base_color = base.device.create_texture_with_data(
                //     &base.queue,
                //     &wgpu::TextureDescriptor { label: None },
                // );

                Primitive { mesh }
            })
            .collect::<Vec<_>>();

        Model { primitives }
    }
}

impl SceneRenderer {
    pub fn render(&mut self, params: &RenderParams<'_, '_, Scene>) -> wgpu::CommandBuffer {
        let RenderParams { base, frame, .. } = params;

        let mut encoder = base
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("test"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&self.g_buffer_pipeline);
            rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
        }

        encoder.finish()
    }
}
