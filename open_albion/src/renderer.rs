
use std::mem;
use std::iter;
use std::num;

use winit::window::Window;
use winit::dpi::PhysicalSize;

use glam::{Mat4,Vec4};

use thunderdome::{Arena,Index as ArenaIdx};

use crate::State;

macro_rules! shader_module {
    ($d:expr, $( $in:tt )*) => {
        $d.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            flags: wgpu::ShaderFlags::empty(),
            source: wgpu::ShaderSource::SpirV(
                std::borrow::Cow::from(&vk_shader_macros::include_glsl!($( $in )*)[..])
            ),
        })
    }
}


pub struct Renderer<'a> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    size: PhysicalSize<u32>,
    format: wgpu::TextureFormat,
    swap_chain: wgpu::SwapChain,
    material_layout: wgpu::BindGroupLayout,

    bufs: Arena<wgpu::Buffer>,
    textures: Arena<wgpu::Texture>,
    samplers: Arena<wgpu::Sampler>,
    materials: Arena<Material>,
    meshes: Vec<Mesh<'a>>,
    camera: wgpu::Buffer,

    albedo_target: wgpu::Texture,
    // TODO: Replace with depth buffer and compute positions in the shader.
    // position_target: wgpu::Texture,
    // normal_target: wgpu::Texture,

    g_buffer_pipeline: wgpu::RenderPipeline,
    lighting_pipeline: wgpu::RenderPipeline,
}

pub struct Material {
    base_color_texture: ArenaIdx,
    // texture_ref: ArenaIdx,
    // uniform_refs: Vec<ArenaIdx>,
    // sampler_refs: Vec<ArenaIdx>,
    bind_group: wgpu::BindGroup,
}

pub struct Mesh<'a> {
    positions_ref: wgpu::BufferSlice<'a>,
    normals_ref: wgpu::BufferSlice<'a>,
    tex_coords_ref: wgpu::BufferSlice<'a>,
    indices_ref: Option<(wgpu::BufferSlice<'a>, wgpu::IndexFormat)>,
    material_ref: ArenaIdx,
    len: u32,
}

impl Renderer<'_> {
    pub async fn create(window: &Window) -> Renderer<'_> {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        }, None).await.unwrap();

        let size = window.inner_size();
        let format = wgpu::TextureFormat::Bgra8Unorm;

        let swap_chain = device.create_swap_chain(&surface, &wgpu::SwapChainDescriptor {
            format,
            height: size.height,
            width: size.width,
            present_mode: wgpu::PresentMode::Mailbox,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        });

        let camera_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera"),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: mem::size_of::<Mat4>() as wgpu::BufferAddress,
            mapped_at_creation: false,
        });

        let albedo_target = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("albedo_target"),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth: 1,
            }
        });

        // let position_target = device.create_texture(&wgpu::TextureDescriptor {
        //     label: Some("position_target"),
        //     mip_level_count: 1,
        //     sample_count: 1,
        //     dimension: wgpu::TextureDimension::D2,
        //     format: wgpu::TextureFormat::Rgba8UnormSrgb,
        //     usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        //     size: wgpu::Extent3d {
        //         width: size.width,
        //         height: size.height,
        //         depth: 1,
        //     }
        // });

        // let normal_target =  device.create_texture(&wgpu::TextureDescriptor {
        //     label: Some("normal_target"),
        //     mip_level_count: 1,
        //     sample_count: 1,
        //     dimension: wgpu::TextureDimension::D2,
        //     format: wgpu::TextureFormat::Rgba8UnormSrgb,
        //     usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
        //     size: wgpu::Extent3d {
        //         width: size.width,
        //         height: size.height,
        //         depth: 1,
        //     }
        // });

        let g_buffer_pipeline = Self::create_g_buffer_pipeline(&device);
        let lighting_pipeline = Self::create_lighting_pipeline(&device);

        let material_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: false,
                        comparison: false,
                    },
                    count: None,
                },
            ]
        });

        Renderer {
            device,
            queue,
            surface,
            size,
            format,
            swap_chain,
            material_layout,

            vertex_bufs: Arena::new(),
            index_bufs: Arena::new(),
            uniform_bufs: Arena::new(),
            camera_buf,
            textures: Arena::new(),
            samplers: Arena::new(),
            // bind_groups: Arena::new(),
            meshes: Vec::new(),
            materials: Arena::new(),

            albedo_target,
            // position_target,
            // normal_target,

            g_buffer_pipeline,
            lighting_pipeline,
        }
    }

    pub fn create_g_buffer_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ]
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            push_constant_ranges: &[],
            bind_group_layouts: &[ &bind_group_layout ],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None, // TODO: Enable
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            vertex: wgpu::VertexState {
                module: &shader_module!(device, "shaders/g_buffer.vert", kind: vert),
                entry_point: "main",
                buffers: &[
                    // wgpu::VertexBufferLayout {}
                ]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module!(device, "shaders/g_buffer.frag", kind: frag),
                entry_point: "main",
                targets: &[
                    // wgpu::ColorTargetState {}
                ]
            }),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }
        })
    }

    pub fn load_resources(&mut self) {

    }

    pub fn create_lighting_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ]
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            push_constant_ranges: &[],
            bind_group_layouts: &[ &bind_group_layout ],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None, // TODO: Enable
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            vertex: wgpu::VertexState {
                module: &shader_module!(device, "shaders/lighting.vert", kind: vert),
                entry_point: "main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 12 as wgpu::BufferAddress,
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float3],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: 12 as wgpu::BufferAddress,
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float3],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: 8 as wgpu::BufferAddress,
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float2],
                    },
                ]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module!(device, "shaders/lighting.frag", kind: frag),
                entry_point: "main",
                targets: &[
                    // wgpu::ColorTargetState {}
                ]
            }),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }
        })
    }

    pub fn render(&mut self, state: &State) {

        // Present the graphics data that was just updated
        self.draw();
    }

    pub fn draw(&mut self) {
        while let Some(mesh) = self.meshes.pop() {
            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            {
                let base_color_view = self.albedo_target.create_view(&wgpu::TextureViewDescriptor::default());

                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    depth_stencil_attachment: None,
                    color_attachments: &[
                        wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &base_color_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: true,
                            }
                        },
                    ]
                });

                rpass.set_pipeline(&self.g_buffer_pipeline);

                if let Some(material) = self.materials.get(mesh.material_ref) {
                    rpass.set_bind_group(0, &material.bind_group, &[]);
                } else {
                    eprintln!("Dropped mesh that is missing its material.");
                    continue
                }

                // for (i, bind_group_ref) in mesh.bind_group_refs.iter().enumerate() {
                //     if let Some(bind_group) = self.bind_groups.get(*bind_group_ref) {
                //         rpass.set_bind_group(i as u32, bind_group, &[]);
                //     } else {
                //         eprintln!("Dropped mesh that is missing a bind group.");
                //         continue
                //     }
                // }

                // for (i, vertex_ref) in mesh.vertices_ref.iter().enumerate() {
                //     if let Some(vertex_buf) = self.vertex_bufs.get(*vertex_ref) {
                //         rpass.set_vertex_buffer(i as u32, vertex_buf.0.slice(..));
                //     } else {
                //         eprintln!("Dropped mesh that is missing its vertex buffer.");
                //         continue;
                //     }
                // }

                if let Some(indices_ref) = &mesh.indices_ref {
                    if let Some(index_buf) = self.index_bufs.get(*indices_ref) {
                        rpass.set_index_buffer(index_buf.0.slice(..), index_buf.1);
                        rpass.draw_indexed(0..mesh.len, 0, 0..1);
                    } else {
                        eprintln!("Dropped mesh that is missing its index buffer.");
                        continue;
                    }
                } else {
                    rpass.draw(0..mesh.len, 0..1);
                }

            }

            self.queue.submit(iter::once(encoder.finish()));
        }
    }

    // pub fn update_swap_chain(&mut self, window: &Window) {
    //     let size = window.inner_size();

    //     self.size = size;

    //     self.swap_chain = self.device.create_swap_chain(&self.surface, &wgpu::SwapChainDescriptor {
    //         usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
    //         format: wgpu::TextureFormat::Bgra8Unorm,
    //         width: size.width,
    //         height: size.height,
    //         present_mode: wgpu::PresentMode::Mailbox,
    //     });
    // }

    // pub fn update_camera(&mut self, camera: Mat4) {
    //     self.queue.write_buffer(&self.camera, 0, bytemuck::cast_slice(camera.as_ref()))
    // }
}