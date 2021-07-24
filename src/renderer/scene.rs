use std::convert::TryInto;
use std::fs::File;
use std::mem;
use std::num::NonZeroU64;

use crate::{include_glsl, Mesh, RendererBase, State};

use crevice::std430::{AsStd430, Std430};
use fable_data::Big;
use glam::{Mat4, Vec3, Vec4};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{vertex_attr_array, VertexState};

pub struct SceneRenderer {
    pipeline: wgpu::RenderPipeline,
    mvp_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    graphics_data: fable_data::Big,
    model: Model,
}

pub struct Model {
    primitives: Vec<Primitive>,
}

pub struct Primitive {
    mesh: Mesh,
    material: Material,
}

pub struct Material {
    // base_color: wgpu::Texture,
}

impl SceneRenderer {
    pub fn create(base: &RendererBase, state: &State) -> Self {
        // let mvp_buffer = base.device.create_buffer(&wgpu::BufferDescriptor {
        //     label: None,
        //     size: mem::size_of::<glam::Mat4>().try_into().unwrap(),
        //     usage: wgpu::BufferUsage::UNIFORM,
        //     mapped_at_creation: false,
        // });

        let mvp_matrix =
            Mat4::perspective_infinite_lh(
                90.0f32.to_radians(),
                base.swap_chain_descriptor.width as f32 / base.swap_chain_descriptor.height as f32,
                0.05,
            ) * Mat4::look_at_lh(state.camera_position, Vec3::new(0.0, 0.0, 0.0), Vec3::Z);

        let mvp_matrix: mint::ColumnMatrix4<f32> = mvp_matrix.into();

        let mvp_matrix = mvp_matrix.as_std430();

        let mvp_buffer = base.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: mvp_matrix.as_bytes(),
            usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM,
        });

        let bind_group_layout =
            base.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: Some(64.try_into().unwrap()),
                        },
                        count: None,
                    }],
                });

        let bind_group = base.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: mvp_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = base
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("scene_render_pipeline_layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = base
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("scene_render_pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &base
                        .device
                        .create_shader_module(&include_glsl!("src/shaders/scene.vert", kind: vert)),
                    entry_point: "main",
                    buffers: &[wgpu::VertexBufferLayout {
                        step_mode: wgpu::InputStepMode::Vertex,
                        array_stride: mem::size_of::<fable_data::Vertex>().try_into().unwrap(),
                        attributes: &wgpu::vertex_attr_array![
                            0 => Float32x4,
                            1 => Float32x4,
                            2 => Float32x2
                        ],
                    }],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &base
                        .device
                        .create_shader_module(&include_glsl!("src/shaders/scene.frag", kind: frag)),
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8Unorm,
                        blend: None,
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState::default(),
                multisample: wgpu::MultisampleState::default(),
                depth_stencil: None,
            });

        let graphics_path = state.fable_dir.join("data/graphics/graphics.big");

        println!("{:?}", graphics_path);

        let mut graphics_file = File::open(&graphics_path).unwrap();

        let graphics_data =
            Big::decode_reader_with_path(&mut graphics_file, &graphics_path).unwrap();

        let model = Self::load_default_model(&base, &graphics_data, &mut graphics_file);

        Self {
            pipeline,
            mvp_buffer,
            graphics_data,
            bind_group,
            model,
        }
    }

    fn load_default_model(
        base: &RendererBase,
        graphics_data: &fable_data::Big,
        graphics_file: &mut File,
    ) -> Model {
        let model_entry =
            &graphics_data.entries[*graphics_data.entries_by_name.get("MESH_CARROT_02").unwrap()];

        let mut model_data = vec![0; model_entry.data_size as usize];

        fable_data::Big::read_entry(graphics_file, &model_entry, &mut model_data)
            .expect("read entry");

        let model_info = if let fable_data::BigInfo::Mesh(model_info) = &model_entry.info {
            Some(model_info)
        } else {
            None
        }
        .unwrap();

        let model = fable_data::Model::decode(&model_data, &model_info).expect("decode model");

        let primitives = model
            .primitives
            .iter()
            .map(|primitive| {
                let vertex_buffer = base.device.create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&primitive.vertices),
                    usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::VERTEX,
                });

                let index_buffer = Some(base.device.create_buffer_init(&BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&primitive.indices),
                    usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::INDEX,
                }));

                let mesh = Mesh {
                    vertex_buffer,
                    index_buffer,
                    count: primitive.index_count,
                };

                // let base_color = base.device.create_texture_with_data(
                //     &base.queue,
                //     &wgpu::TextureDescriptor {
                //         label: None
                //     },
                //     &[],
                // );

                let material = Material {};

                Primitive { mesh, material }
            })
            .collect::<Vec<_>>();

        Model { primitives }
    }

    pub fn write_mvp_matrix(&mut self, base: &RendererBase, state: &State) {
        let focal_point = state.camera_position + (state.camera_rotation * Vec3::X);

        let proj = Mat4::perspective_infinite_lh(
            90.0f32.to_radians(),
            base.swap_chain_descriptor.width as f32 / base.swap_chain_descriptor.height as f32,
            0.05,
        );

        let look_at = Mat4::look_at_lh(state.camera_position, focal_point, Vec3::Z);

        let mvp_matrix: mint::ColumnMatrix4<f32> = (proj * look_at).into();
        let mvp_matrix = mvp_matrix.as_std430();

        base.queue
            .write_buffer(&self.mvp_buffer, 0, mvp_matrix.as_bytes());
    }

    pub fn render(
        &mut self,
        base: &RendererBase,
        frame: &wgpu::SwapChainFrame,
        state: &State,
    ) -> wgpu::CommandBuffer {
        let mut encoder = base
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.write_mvp_matrix(&base, &state);

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
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

            rpass.set_pipeline(&self.pipeline);

            let primitive = &self.model.primitives.first().unwrap();

            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_vertex_buffer(0, primitive.mesh.vertex_buffer.slice(..));

            if let Some(index_buffer) = &primitive.mesh.index_buffer {
                rpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                rpass.draw_indexed(0..primitive.mesh.count, 0, 0..1);
            } else {
                rpass.draw(0..primitive.mesh.count, 0..1);
            }
        }

        encoder.finish()
    }
}
